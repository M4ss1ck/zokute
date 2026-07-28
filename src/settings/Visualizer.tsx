import { Label, Slider, SliderFill, SliderOutput, SliderThumb, SliderTrack } from "react-aria-components";
import type { SectionConfig, MergedConfig } from "../useStats";
import { ColorControl } from "./Color";
import { SettingSwitch } from "./SettingSwitch";
import { SettingsToggleGroup } from "./ToggleGroup";

const COLOR_MODES = [
  { id: "solid", label: "Solid" },
  { id: "gradient", label: "Gradient" },
];

const GRADIENT_DIRECTIONS = [
  { id: "horizontal", label: "Horizontal" },
  { id: "vertical", label: "Vertical" },
];

const FPS_OPTIONS = [
  { id: "30", label: "30" },
  { id: "60", label: "60" },
];

interface Props {
  config: MergedConfig;
  onChange: (next: MergedConfig) => void;
}

function isVisualizer(section: SectionConfig) {
  return section.id === "spectrum" || section.id === "ring";
}

function NumberField({ label, value, onChange, min, max, step }: {
  label: string;
  value: number;
  onChange: (v: number) => void;
  min?: number;
  max?: number;
  step?: number;
}) {
  return (
    <div className="settingsSlider">
      <Label className="settingsRowLabel">{label}</Label>
      <input
        className="settingsValue"
        type="number"
        style={{ width: "5em", textAlign: "right", background: "transparent", border: "none", color: "inherit", font: "inherit" }}
        value={value}
        min={min}
        max={max}
        step={step ?? 1}
        onChange={(e) => onChange(Number(e.target.value))}
      />
    </div>
  );
}

function VizSlider({ label, value, onChange, min, max, step, format }: {
  label: string;
  value: number;
  onChange: (v: number) => void;
  min: number;
  max: number;
  step: number;
  format?: (v: number) => string;
}) {
  return (
    <Slider className="settingsSlider" minValue={min} maxValue={max} step={step} value={value} onChange={onChange}>
      <Label className="settingsRowLabel">{label}</Label>
      <SliderOutput className="settingsValue">{({ state }) => (format ?? ((v: number) => String(v)))(state.getThumbValue(0))}</SliderOutput>
      <SliderTrack className="settingsSliderTrack">
        <SliderFill className="settingsSliderFill" />
        <SliderThumb className="settingsSliderThumb" />
      </SliderTrack>
    </Slider>
  );
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
            {section.color_mode === "gradient" ? (
              <>
                <ColorControl label={`${instance} second color`} value={section.color_b ?? "#c07100"} onChange={(color_b) => patch(instance, { color_b })} />
                <SettingsToggleGroup
                  label="Gradient direction"
                  options={GRADIENT_DIRECTIONS}
                  value={section.gradient_direction ?? "horizontal"}
                  onChange={(gradient_direction) => patch(instance, { gradient_direction: gradient_direction as "horizontal" | "vertical" })}
                />
              </>
            ) : null}
            <NumberField label="Bar count" value={section.viz_bar_count ?? 64} onChange={(viz_bar_count) => patch(instance, { viz_bar_count })} min={4} max={256} />
            <NumberField label="Min Hz" value={section.viz_min_hz ?? 20} onChange={(viz_min_hz) => patch(instance, { viz_min_hz })} min={1} max={1000} />
            <NumberField label="Max Hz" value={section.viz_max_hz ?? 22000} onChange={(viz_max_hz) => patch(instance, { viz_max_hz })} min={500} max={24000} />
            <VizSlider label="Gain" value={section.viz_gain ?? 1.0} onChange={(viz_gain) => patch(instance, { viz_gain })} min={0.1} max={3} step={0.05} format={(v) => v.toFixed(2)} />
            <VizSlider label="Smoothing" value={section.viz_smoothing ?? 0.8} onChange={(viz_smoothing) => patch(instance, { viz_smoothing })} min={0} max={0.99} step={0.01} format={(v) => `${Math.round(v * 100)}%`} />
            <VizSlider label="Decay" value={section.viz_decay ?? 0.3} onChange={(viz_decay) => patch(instance, { viz_decay })} min={0.05} max={2} step={0.05} format={(v) => v.toFixed(2)} />
            <SettingSwitch isSelected={section.viz_mirror ?? true} onChange={(viz_mirror) => patch(instance, { viz_mirror })}>Mirror</SettingSwitch>
            <NumberField label="Gap" value={section.viz_gap ?? 1} onChange={(viz_gap) => patch(instance, { viz_gap })} min={0} max={20} />
            <SettingSwitch isSelected={section.viz_rounded_caps ?? false} onChange={(viz_rounded_caps) => patch(instance, { viz_rounded_caps })}>Rounded caps</SettingSwitch>
            <SettingsToggleGroup label="FPS" options={FPS_OPTIONS} value={String(section.viz_fps ?? 30)} onChange={(viz_fps) => patch(instance, { viz_fps: Number(viz_fps) })} />
          </div>
        );
      })}
    </section>
  );
}
