import { IconPlus } from "@tabler/icons-react";
import { Button } from "react-aria-components";
import type { MergedConfig, SectionConfig } from "../useStats";
import { PanelCard } from "./PanelCard";

interface Props {
  config: MergedConfig;
  onChange: (next: MergedConfig) => void;
}

// Same shape as Sections.instanceLabel, but the taken-set also covers every
// child, because the Rust side uses instance as the window label.
function instanceLabel(id: string, sections: SectionConfig[]) {
  const labels = new Set(
    sections.flatMap((section) => [
      section.instance ?? section.id,
      ...(section.children ?? []).map((child) => child.instance ?? child.id),
    ]),
  );
  if (!labels.has(id)) return id;
  let suffix = 2;
  while (labels.has(`${id}-${suffix}`)) suffix += 1;
  return `${id}-${suffix}`;
}

export function PanelPreferences({ config, onChange }: Props) {
  const panels = config.sections.filter((section) => section.id === "panel");
  function addPanel() {
    const last = [...config.sections].reverse().find((section) => section.id === "panel");
    const panel: SectionConfig = {
      id: "panel",
      instance: instanceLabel("panel", config.sections),
      enabled: true,
      show_header: true,
      monitor: 0,
      x: (last?.x ?? 0) + 24,
      y: (last?.y ?? 0) + 24,
      width: 360,
      scale: 1,
      children: [],
      panel_gap: 4,
      panel_padding: 8,
      panel_dividers: true,
    };
    onChange({ ...config, sections: [...config.sections, panel] });
  }
  return (
    <section className="settingsCard" aria-labelledby="panels-title">
      <header className="settingsCardHeader">
        <h2 id="panels-title">Panels</h2>
        <p>Panel containers hold several widgets.</p>
        <Button
          aria-label="Add panel"
          className="settingsAddButton"
          onPress={addPanel}
          style={{ justifySelf: "end" }}
        >
          <span>Panel</span>
          <IconPlus aria-hidden="true" />
        </Button>
      </header>
      {panels.length === 0 && (
        <p className="settingsEmpty">No panels configured.</p>
      )}
      {panels.map((panel) => (
        <PanelCard key={panel.instance ?? panel.id} panel={panel} config={config} onChange={onChange} />
      ))}
    </section>
  );
}
