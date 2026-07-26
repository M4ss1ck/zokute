import {
  Label,
  Slider,
  SliderFill,
  SliderOutput,
  SliderThumb,
  SliderTrack,
} from "react-aria-components";
import type { SectionConfig, StatsConfig } from "../useStats";

interface Props {
  config: StatsConfig;
  onChange: (next: StatsConfig) => void;
}

// Zoom is deliberately not a drag gesture. Deriving it from the dragged width
// meant widening a widget enlarged its text instead of spreading its columns,
// and left no way to zoom a widget without also moving its edges.
export function SizePreferences({ config, onChange }: Props) {
  const enabled = config.sections.filter((section) => section.enabled);
  if (enabled.length === 0) return null;
  function patch(instance: string, changes: Partial<SectionConfig>) {
    onChange({
      ...config,
      sections: config.sections.map((section) =>
        (section.instance ?? section.id) === instance ? { ...section, ...changes } : section,
      ),
    });
  }
  return (
    <section className="settingsCard" aria-labelledby="sizes-title">
      <header className="settingsCardHeader">
        <h2 id="sizes-title">Size</h2>
        <p>Zoom a widget without changing the space it occupies.</p>
      </header>
      {enabled.map((section) => {
        const instance = section.instance ?? section.id;
        return (
          <Slider
            className="settingsSlider"
            key={instance}
            minValue={0.25}
            maxValue={4}
            step={0.05}
            value={section.scale ?? 1}
            onChange={(scale) => patch(instance, { scale })}
          >
            <Label className="settingsRowLabel">{instance}</Label>
            <SliderOutput className="settingsValue">
              {({ state }) => `${Math.round(state.getThumbValue(0) * 100)}%`}
            </SliderOutput>
            <SliderTrack className="settingsSliderTrack">
              <SliderFill className="settingsSliderFill" />
              <SliderThumb className="settingsSliderThumb" />
            </SliderTrack>
          </Slider>
        );
      })}
    </section>
  );
}
