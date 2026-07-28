import { IconDeviceDesktop } from "@tabler/icons-react";
import type { Stats } from "../useStats";

interface Props {
  stats: Stats;
}

export function SystemWidget({ stats }: Props) {
  const fields = stats.profile.system_fields.flatMap((fieldId) =>
    stats.system_fields.filter((field) => field.id === fieldId),
  );
  const showHeader = stats.profile.sections.find((section) => section.id === "system")?.show_header ?? true;
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
