#!/usr/bin/env bash
# M9 performance gate: report the running cost of Zokute.
# CPU: interval delta of utime+stime across descendants present in both snapshots,
# normalized by /proc/stat total tick delta. PSS memory: sum of /proc/<pid>/smaps_rollup.
set -uo pipefail

label="${1:-unlabelled}"
root_pid="$(pgrep -x zokute | head -1 || true)"

printf '=== zokute resources: %s (%s) ===\n' "$label" "$(date -Is)"

if [ -z "$root_pid" ]; then
  echo "zokute is not running"
  exit 0
fi

discover_descendants() {
  local root=$1
  local queue=("$root")
  local head=0
  while [ $head -lt ${#queue[@]} ]; do
    local pid="${queue[$head]}"
    head=$((head + 1))
    local children_file="/proc/$pid/task/$pid/children"
    if [ -r "$children_file" ]; then
      for child in $(cat "$children_file" 2>/dev/null); do
        queue+=("$child")
      done
    fi
  done
  printf '%s\n' "${queue[@]}"
}

# utime+stime only — cutime/cstime cover exited children and skew deltas.
read_pid_ticks() {
  local pid=$1
  local stat_file="/proc/$pid/stat"
  if [ ! -r "$stat_file" ]; then printf '0'; return; fi
  local after_comm
  after_comm="$(cat "$stat_file" 2>/dev/null)"
  after_comm="${after_comm##*)}"
  echo "$after_comm" | awk '{ print $12 + $13 }'
}

read_total_ticks() {
  awk '/^cpu / { print $2+$3+$4+$5+$6+$7+$8+$9; exit }' /proc/stat
}

# First snapshot: discover tree, read ticks.
before_pids=()
while IFS= read -r pid; do before_pids+=("$pid"); done < <(discover_descendants "$root_pid")

before_ticks=()
for pid in "${before_pids[@]}"; do
  before_ticks+=("$(read_pid_ticks "$pid")")
done
before_total="$(read_total_ticks)"

sleep 1

# Second snapshot: rediscover tree for intersection.
after_pids=()
while IFS= read -r pid; do after_pids+=("$pid"); done < <(discover_descendants "$root_pid")

# Build PID→index hash for before snapshot to intersect.
declare -A before_index
for (( i=0; i<${#before_pids[@]}; i++ )); do
  before_index[${before_pids[$i]}]=$i
done

pid_delta=0
matched=0
for (( i=0; i<${#after_pids[@]}; i++ )); do
  pid="${after_pids[$i]}"
  tick_after="$(read_pid_ticks "$pid")"
  if [ -n "${before_index[$pid]+x}" ]; then
    tick_before="${before_ticks[${before_index[$pid]}]}"
    if (( tick_after >= tick_before )); then
      pid_delta=$(( pid_delta + tick_after - tick_before ))
      matched=$(( matched + 1 ))
    fi
  fi
done

after_total="$(read_total_ticks)"
total_delta=$(( after_total - before_total ))

cpu_pct="0.0"
if (( total_delta > 0 )); then
  cpu_pct="$(awk "BEGIN { printf \"%.1f\", ($pid_delta / $total_delta) * 100 }")"
fi

# PSS and display use current (post-wait) tree.
total_pss_kb=0
for pid in "${after_pids[@]}"; do
  pss="$(awk '/^Pss:/ { print $2; exit }' "/proc/$pid/smaps_rollup" 2>/dev/null || echo 0)"
  total_pss_kb=$(( total_pss_kb + pss ))
done
ram_mib="$(awk "BEGIN { printf \"%.1f\", $total_pss_kb / 1024 }")"

echo
echo "--- process tree ---"
ps -o pid,ppid,rss,pcpu,comm $(printf ' -p %s' "${after_pids[@]}") 2>/dev/null

echo
echo "--- totals ---"
printf 'processes  %d\ncpu        %s %%\nram        %s MiB (PSS)\n' \
  "${#after_pids[@]}" "$cpu_pct" "$ram_mib"

echo
echo "--- webviews ---"
webview_count=0
for pid in "${after_pids[@]}"; do
  if grep -q '^WebKitWeb' "/proc/$pid/comm" 2>/dev/null; then
    webview_count=$(( webview_count + 1 ))
  fi
done
printf 'count      %s\n' "$webview_count"

echo
echo "--- collector / audio / plugins ---"
if command -v zokute >/dev/null 2>&1; then
  zokute diagnostics 2>/dev/null \
    | grep -E '"(interval_ms|active|degraded|detection|behavior|section_count)"' \
    || echo "diagnostics unavailable"
else
  echo "zokute CLI not on PATH"
fi

echo
echo "--- threads ---"
printf 'count      %s\n' "$(ls "/proc/$root_pid/task" 2>/dev/null | wc -l)"
