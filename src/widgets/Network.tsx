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
  return (
    <section className="panel">
      <header className="panelHeader">
        <span className="panelTitleGroup">
          <IconNetwork className="panelIcon" />
          <span className="panelTitle">Network</span>
        </span>
      </header>
      <div className="metric">
        <span className="metricLabel">Down</span>
        <span className="metricValue">{formatRate(stats.network.down_bytes_per_second)}</span>
      </div>
      <Sparkline values={history.networkDown} />
      <div className="metric">
        <span className="metricLabel">Up</span>
        <span className="metricValue">{formatRate(stats.network.up_bytes_per_second)}</span>
      </div>
      <Sparkline values={history.networkUp} />
    </section>
  );
}
