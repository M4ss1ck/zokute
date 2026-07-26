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
  const fields = stats.config.system_fields.flatMap((fieldId) =>
    stats.system_fields.filter((field) => field.id === fieldId),
  );
  const showHeader = stats.config.sections.find((section) => section.id === "system")?.show_header ?? true;
  return (
    <section className="panel">
      {showHeader ? (
        <header className="panelHeader">
          <span className="panelTitleGroup">
            <IconDeviceDesktop className="panelIcon" />
            <span className="panelTitle">System</span>
          </span>
        </header>
      ) : null}
      {fields.map((field, index) => (
        <div className="metric" key={`${field.id}-${index}`}>
          <span className="metricLabel">{field.label}</span>
          <span className="metricValue">{field.value}</span>
        </div>
      ))}
    </section>
  );
}
