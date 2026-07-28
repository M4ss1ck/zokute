import type { SectionConfig, MergedConfig } from "../useStats";
import { IconPlus } from "@tabler/icons-react";
import { Button } from "react-aria-components";
import { currentMonitor } from "@tauri-apps/api/window";
import { RING_SIZE } from "../widgets/Ring";
import { SPECTRUM_HEIGHT } from "../widgets/Spectrum";

interface Props {
  config: MergedConfig;
  onChange: (next: MergedConfig) => void;
}

const widgetKinds = ["system", "cpu", "memory", "disk", "network", "spectrum", "ring", "clock", "date"] as const;

function instanceLabel(id: string, sections: SectionConfig[]) {
  const labels = new Set(sections.map((section) => section.instance ?? section.id));
  if (!labels.has(id)) return id;
  let suffix = 2;
  while (labels.has(`${id}-${suffix}`)) suffix += 1;
  return `${id}-${suffix}`;
}

async function seededPlacement(id: string) {
  if (id !== "spectrum" && id !== "ring") return null;
  const monitor = await currentMonitor();
  const scale = monitor?.scaleFactor ?? 1;
  const width = Math.round((monitor?.size.width ?? 1920) / scale);
  const height = Math.round((monitor?.size.height ?? 1080) / scale);
  if (id === "spectrum") return { x: 0, y: height - SPECTRUM_HEIGHT, width, height: SPECTRUM_HEIGHT };
  return { x: Math.round((width - RING_SIZE) / 2), y: Math.round((height - RING_SIZE) / 2), width: RING_SIZE, height: RING_SIZE };
}

export function SectionToggles({ config, onChange }: Props) {
  async function add(id: string) {
    const source = [...config.sections].reverse().find((section) => section.id === id);
    const seeded = await seededPlacement(id);
    const section: SectionConfig = {
      id,
      instance: instanceLabel(id, config.sections),
      enabled: true,
      show_header: source?.show_header ?? true,
      monitor: source?.monitor ?? 0,
      x: seeded?.x ?? (source?.x ?? 0) + 24,
      y: seeded?.y ?? (source?.y ?? 0) + 24,
      width: seeded?.width ?? source?.width ?? 360,
      height: seeded?.height ?? source?.height,
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
              onPress={() => void add(id)}
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
