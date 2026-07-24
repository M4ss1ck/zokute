import type { Stats } from "../useStats";

interface Props {
  stats: Stats;
}

function clampPercent(used: number, total: number) {
  if (!Number.isFinite(used) || !Number.isFinite(total) || total <= 0) return 0;
  return Math.max(0, Math.min(100, (used / total) * 100));
}

function formatBytes(bytes: number) {
  const units = ["B", "KB", "MB", "GB", "TB"];
  let size = Math.max(0, bytes);
  let index = 0;
  while (size >= 1024 && index < units.length - 1) {
    size /= 1024;
    index += 1;
  }
  return `${size.toFixed(size >= 10 || index === 0 ? 0 : 1)} ${units[index]}`;
}

function usageLabel(used: number, total: number) {
  return `${formatBytes(used)} / ${formatBytes(total)}`;
}

export function MemoryWidget({ stats }: Props) {
  const memoryPercent = clampPercent(stats.memory.used_bytes, stats.memory.total_bytes);
  const swapPercent = clampPercent(stats.memory.swap_used_bytes, stats.memory.swap_total_bytes);
  return (
    <section className="panel">
      <header className="panelHeader">
        <span className="panelTitle">Memory</span>
        <span className="panelValue">{memoryPercent.toFixed(1)}%</span>
      </header>
      <div className="metric">
        <span className="metricLabel">Used</span>
        <span className="metricValue">{usageLabel(stats.memory.used_bytes, stats.memory.total_bytes)}</span>
      </div>
      <div className="barTrack" aria-hidden="true">
        <div className="barFill" style={{ width: `${memoryPercent}%` }} />
      </div>
      <div className="metric">
        <span className="metricLabel">Swap</span>
        <span className="metricValue">{usageLabel(stats.memory.swap_used_bytes, stats.memory.swap_total_bytes)}</span>
      </div>
      <div className="barTrack" aria-hidden="true">
        <div className="barFill" style={{ width: `${swapPercent}%` }} />
      </div>
    </section>
  );
}
