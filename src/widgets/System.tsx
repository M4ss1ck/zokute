import { IconDeviceDesktop } from "@tabler/icons-react";
import type { Stats, SystemField } from "../useStats";

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

function systemFieldForId(stats: Stats, id: string): SystemField | null {
  if (id === "uptime") {
    return { id, label: "Uptime", value: formatUptime(stats.uptime) };
  }
  return stats.system_fields.find((field) => field.id === id) ?? null;
}

export function SystemWidget({ stats }: Props) {
  const fields = stats.config.system_fields.map((fieldId) => systemFieldForId(stats, fieldId)).filter((field): field is SystemField => field !== null);
  return (
    <section className="panel">
      <header className="panelHeader">
        <span className="panelTitleGroup">
          <IconDeviceDesktop className="panelIcon" />
          <span className="panelTitle">System</span>
        </span>
      </header>
      {fields.map((field) => (
        <div className="metric" key={field.id}>
          <span className="metricLabel">{field.label}</span>
          <span className="metricValue">{field.value}</span>
        </div>
      ))}
    </section>
  );
}
