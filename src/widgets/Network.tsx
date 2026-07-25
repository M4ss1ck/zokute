import { IconNetwork } from "@tabler/icons-react";
import { Sparkline } from "../viz/Sparkline";
import type { Stats, StatsHistory } from "../useStats";

interface Props {
  stats: Stats;
  history: StatsHistory;
}

function formatRate(bytesPerSecond: number) {
  const units = ["B/s", "KB/s", "MB/s", "GB/s"];
  let size = Math.max(0, bytesPerSecond);
  let index = 0;
  while (size >= 1024 && index < units.length - 1) {
    size /= 1024;
    index += 1;
  }
  return `${size.toFixed(size >= 10 || index === 0 ? 0 : 1)} ${units[index]}`;
}

export function NetworkWidget({ stats, history }: Props) {
  const showHeader = stats.config.sections.find((section) => section.id === "network")?.show_header ?? true;
  const rows = [
    { label: "Down", value: formatRate(stats.network.down_bytes_per_second), values: history.networkDown },
    { label: "Up", value: formatRate(stats.network.up_bytes_per_second), values: history.networkUp },
  ];
  return (
    <section className="panel">
      {showHeader ? (
        <header className="panelHeader">
          <span className="panelTitleGroup">
            <IconNetwork className="panelIcon" />
            <span className="panelTitle">Network</span>
          </span>
        </header>
      ) : null}
      <div className="networkStack">
        {rows.map((row) => (
          <div className="networkRow" key={row.label}>
            <div className="metric">
              <span className="metricLabel">{row.label}</span>
              <span className="metricValue">{row.value}</span>
            </div>
            <div className="networkSparklineRow">
              <Sparkline values={row.values} />
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
