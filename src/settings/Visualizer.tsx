import type { SectionConfig, StatsConfig } from "../useStats";
import { ColorControl } from "./Color";
import { SettingsToggleGroup } from "./ToggleGroup";

const COLOR_MODES = [
  { id: "solid", label: "Solid" },
  { id: "gradient", label: "Gradient" },
];

interface Props {
  config: StatsConfig;
  onChange: (next: StatsConfig) => void;
}

function isVisualizer(section: SectionConfig) {
  return section.id === "spectrum" || section.id === "ring";
}

export function VisualizerPreferences({ config, onChange }: Props) {
  const visualizers = config.sections.filter((section) => section.enabled && isVisualizer(section));
  if (visualizers.length === 0) return null;
  function patch(instance: string, changes: Partial<SectionConfig>) {
    onChange({
      ...config,
      sections: config.sections.map((section) =>
        (section.instance ?? section.id) === instance ? { ...section, ...changes } : section,
      ),
    });
  }
  return (
    <section className="settingsCard" aria-labelledby="visualizers-title">
      <header className="settingsCardHeader">
        <h2 id="visualizers-title">Visualizers</h2>
        <p>Colour the bars that follow whatever is playing.</p>
      </header>
      {visualizers.map((section) => {
        const instance = section.instance ?? section.id;
        return (
          <div className="settingsRow" key={instance}>
            <SettingsToggleGroup label="Color mode" options={COLOR_MODES} value={section.color_mode ?? "solid"} onChange={(color_mode) => patch(instance, { color_mode: color_mode as "solid" | "gradient" })} />
            <ColorControl label={`${instance} color`} value={section.color_a ?? "#494137"} onChange={(color_a) => patch(instance, { color_a })} />
            {section.color_mode === "gradient" ? <ColorControl label={`${instance} second color`} value={section.color_b ?? "#c07100"} onChange={(color_b) => patch(instance, { color_b })} /> : null}
          </div>
        );
      })}
    </section>
  );
}
