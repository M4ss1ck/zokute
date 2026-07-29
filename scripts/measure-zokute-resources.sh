#!/usr/bin/env bash
# M9 performance gate: report the running cost of Zokute without capturing any
# user data. Reports process tree, CPU, RSS, WebView count, collector cadence,
# plugin activity, and audio state.
set -uo pipefail

label="${1:-unlabelled}"
root_pid="$(pgrep -x zokute | head -1 || true)"

printf '=== zokute resources: %s (%s) ===\n' "$label" "$(date -Is)"

if [ -z "$root_pid" ]; then
  echo "zokute is not running"
  exit 0
fi

echo
echo "--- process tree ---"
ps -o pid,ppid,rss,pcpu,comm --ppid "$root_pid" --pid "$root_pid" 2>/dev/null

echo
echo "--- totals ---"
ps -o rss=,pcpu= --ppid "$root_pid" --pid "$root_pid" 2>/dev/null | awk '
  { rss += $1; cpu += $2; procs += 1 }
  END { printf "processes  %d\nrss        %.1f MiB\ncpu        %.1f %%\n", procs, rss / 1024, cpu }'

echo
echo "--- webviews ---"
printf 'count      %s\n' "$(pgrep -c -P "$root_pid" -f WebKitWebProcess 2>/dev/null || echo 0)"

echo
echo "--- collector / audio / plugins ---"
# Field names only; diagnostics already excludes note text and plugin output.
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
