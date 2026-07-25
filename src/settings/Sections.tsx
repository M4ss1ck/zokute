import type { SectionConfig, StatsConfig } from "../useStats";
import { IconPlus } from "@tabler/icons-react";
import { Button } from "react-aria-components";

interface Props {
  config: StatsConfig;
  onChange: (next: StatsConfig) => void;
}

const widgetKinds = ["system", "cpu", "memory", "disk", "network"] as const;

function instanceLabel(id: string, sections: SectionConfig[]) {
  const labels = new Set(sections.map((section) => section.instance ?? section.id));
  if (!labels.has(id)) return id;
  let suffix = 2;
  while (labels.has(`${id}-${suffix}`)) suffix += 1;
  return `${id}-${suffix}`;
}

export function SectionToggles({ config, onChange }: Props) {
  function add(id: string) {
    const source = [...config.sections].reverse().find((section) => section.id === id);
    const section: SectionConfig = {
      id,
      instance: instanceLabel(id, config.sections),
      enabled: true,
      show_header: source?.show_header ?? true,
      monitor: source?.monitor ?? 0,
      x: (source?.x ?? 0) + 24,
      y: (source?.y ?? 0) + 24,
      width: source?.width ?? 360,
      scale: source?.scale ?? 1,
    };
    onChange({ ...config, sections: [...config.sections, section] });
  }
  return (
    <section className="settingsCard" aria-labelledby="widgets-title">
      <header className="settingsCardHeader">
        <h2 id="widgets-title">Widgets</h2>
        <p>Add as many monitors as your desktop needs.</p>
      </header>
      <div className="settingsAddGrid">
        {widgetKinds.map((id) => {
          const count = config.sections.filter((section) => section.id === id && section.enabled).length;
          return (
            <Button
              aria-label={`Add ${id} widget (${count} active)`}
              className="settingsAddButton"
              key={id}
              onPress={() => add(id)}
            >
              <span>{id}</span>
              <span className="settingsWidgetCount">{count}</span>
              <IconPlus aria-hidden="true" />
            </Button>
          );
        })}
      </div>
    </section>
  );
}
