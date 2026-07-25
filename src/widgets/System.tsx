import { IconDeviceDesktop } from "@tabler/icons-react";
import type { Stats } from "../useStats";

interface Props {
  stats: Stats;
}

function formatUptime(totalSeconds: number) {
  const seconds = Math.max(0, Math.floor(totalSeconds));
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  return days > 0 ? `${days}d ${hours}h ${minutes}m` : hours > 0 ? `${hours}h ${minutes}m` : `${minutes}m`;
}

export function SystemWidget({ stats }: Props) {
  return (
    <section className="panel">
      <header className="panelHeader">
        <span className="panelTitleGroup">
          <IconDeviceDesktop className="panelIcon" />
          <span className="panelTitle">System</span>
        </span>
      </header>
      <div className="metric">
        <span className="metricLabel">Hostname</span>
        <span className="metricValue">{stats.hostname || "Unknown"}</span>
      </div>
      <div className="metric">
        <span className="metricLabel">Uptime</span>
        <span className="metricValue">{formatUptime(stats.uptime)}</span>
      </div>
    </section>
  );
}
