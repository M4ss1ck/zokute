import { IconServer2 } from "@tabler/icons-react";
import { Arc } from "../viz/Arc";
import { formatBytes } from "../format";
import type { Stats } from "../useStats";

interface Props {
  stats: Stats;
}

function clampPercent(used: number, total: number) {
  if (!Number.isFinite(used) || !Number.isFinite(total) || total <= 0) return 0;
  return Math.max(0, Math.min(100, (used / total) * 100));
}

function usageLabel(used: number, total: number, mode: "binary" | "decimal") {
  return `${formatBytes(used, mode)} / ${formatBytes(total, mode)}`;
}

export function MemoryWidget({ stats }: Props) {
  const memoryPercent = clampPercent(stats.memory.used_bytes, stats.memory.total_bytes);
  const hasSwap = stats.memory.swap_total_bytes > 0;
  const showHeader = stats.config.sections.find((section) => section.id === "memory")?.show_header ?? true;
  const byteMode = (stats.config.byte_format ?? "binary") as "binary" | "decimal";
  const items = [
    { label: "Memory", used: stats.memory.used_bytes, total: stats.memory.total_bytes, percent: memoryPercent },
    ...(hasSwap
      ? [{ label: "Swap", used: stats.memory.swap_used_bytes, total: stats.memory.swap_total_bytes, percent: clampPercent(stats.memory.swap_used_bytes, stats.memory.swap_total_bytes) }]
      : []),
  ];
  const gridClassName = items.length === 1 ? "memoryGrid memoryGrid--single" : "memoryGrid";
  return (
    <section className="panel">
      {showHeader ? (
        <header className="panelHeader">
          <span className="panelTitleGroup">
            <IconServer2 className="panelIcon" />
            <span className="panelTitle">Memory</span>
          </span>
          <span className="panelValue">{memoryPercent.toFixed(1)}%</span>
        </header>
      ) : null}
      <div className={gridClassName}>
        {items.map((item) => (
          <div className="memoryItem" key={item.label}>
            <Arc percent={item.percent} />
            <div className="memoryStack">
              <span className="metricLabel">{item.label}</span>
              <span className="metricValue">{usageLabel(item.used, item.total, byteMode)}</span>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
