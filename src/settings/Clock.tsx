import { Radio, RadioGroup } from "react-aria-components";
import type { SectionConfig, StatsConfig } from "../useStats";
import { CLOCK_WIDTH } from "../widgets/Clock";
import { ColorControl } from "./Color";
import { SettingSwitch } from "./SettingSwitch";

interface Props {
  config: StatsConfig;
  onChange: (next: StatsConfig) => void;
}

export function ClockPreferences({ config, onChange }: Props) {
  const clocks = config.sections.filter((section) => section.enabled && section.id === "clock");
  if (clocks.length === 0) return null;
  function patch(instance: string, changes: Partial<SectionConfig>) {
    onChange({
      ...config,
      sections: config.sections.map((section) =>
        (section.instance ?? section.id) === instance ? { ...section, ...changes } : section,
      ),
    });
  }
  return (
    <section className="settingsCard" aria-labelledby="clocks-title">
      <header className="settingsCardHeader">
        <h2 id="clocks-title">Clocks</h2>
        <p>Shape how each clock reads the time.</p>
      </header>
      {clocks.map((section) => {
        const instance = section.instance ?? section.id;
        return (
          <div className="settingsCardBody" key={instance}>
            <RadioGroup
              aria-label={`${instance} font`}
              value={section.clock_font ?? "mono"}
              onChange={(clock_font) => patch(instance, { clock_font: clock_font as "mono" | "sans" })}
            >
              <Radio value="mono">JetBrains Mono</Radio>
              <Radio value="sans">IBM Plex Sans</Radio>
            </RadioGroup>
            <ColorControl
              label={`${instance} color`}
              value={section.clock_color ?? config.text_color ?? "#292824"}
              onChange={(clock_color) => patch(instance, { clock_color })}
            />
            <SettingSwitch
              isSelected={section.clock_seconds ?? false}
              onChange={(clock_seconds) => patch(instance, { clock_seconds })}
            >
              Show seconds
            </SettingSwitch>
            <SettingSwitch
              isSelected={section.clock_24h ?? false}
              onChange={(clock_24h) => patch(instance, { clock_24h })}
            >
              24-hour time
            </SettingSwitch>
            <SettingSwitch
              isDisabled={section.clock_24h ?? false}
              isSelected={section.clock_ampm ?? true}
              onChange={(clock_ampm) => patch(instance, { clock_ampm })}
            >
              Show AM/PM
            </SettingSwitch>
            <SettingSwitch
              isSelected={section.clock_pad ?? true}
              onChange={(clock_pad) => patch(instance, { clock_pad })}
            >
              Padded digits
            </SettingSwitch>
            {/* A width tuned for one layout is wrong for the other, so the box
                goes back to that layout's default and re-fits its content. */}
            <RadioGroup
              aria-label={`${instance} layout`}
              value={section.clock_layout ?? "row"}
              onChange={(value) => {
                const clock_layout = value as "row" | "column";
                patch(instance, { clock_layout, width: CLOCK_WIDTH[clock_layout], height: undefined });
              }}
            >
              <Radio value="row">Row</Radio>
              <Radio value="column">Column</Radio>
            </RadioGroup>
          </div>
        );
      })}
    </section>
  );
}
