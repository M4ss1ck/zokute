import { invoke } from "@tauri-apps/api/core";
import type { MergedConfig, SectionConfig } from "../useStats";
import { CLOCK_WIDTH } from "../widgets/Clock";
import { instanceTitle } from "../instance-title";
import { ColorControl } from "./Color";
import { SettingSwitch } from "./SettingSwitch";
import { SettingsToggleGroup } from "./ToggleGroup";

const FONTS = [
  { id: "mono", label: "JetBrains Mono" },
  { id: "sans", label: "IBM Plex Sans" },
];
const ALIGNMENTS = [
  { id: "left", label: "Left" },
  { id: "center", label: "Center" },
  { id: "right", label: "Right" },
];
const LAYOUTS = [
  { id: "row", label: "Row" },
  { id: "column", label: "Column" },
];

interface Props {
  config: MergedConfig;
  onChange: (next: MergedConfig) => void;
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
          <div className="settingsCardBody settingsInstance" key={instance} role="group" aria-labelledby={`instance-${instance}`}>
            <h3 className="settingsInstanceHeader" id={`instance-${instance}`}>{instanceTitle(instance)}</h3>
            <SettingsToggleGroup
              label="Font"
              options={FONTS}
              value={section.clock_font ?? "mono"}
              onChange={(clock_font) => patch(instance, { clock_font: clock_font as "mono" | "sans" })}
            />
            <ColorControl
              label="Color"
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
            <SettingsToggleGroup
              label="Alignment"
              options={ALIGNMENTS}
              value={section.clock_align ?? "left"}
              onChange={(align) => patch(instance, { clock_align: align as SectionConfig["clock_align"] })}
            />
            {/* A width tuned for one layout is wrong for the other, so the box
                goes back to that layout's default. It travels to the window and
                not into the config: settings runs inside edit mode, which
                recaptures live geometry over anything the file says. */}
            <SettingsToggleGroup
              label="Layout"
              options={LAYOUTS}
              value={section.clock_layout ?? "row"}
              onChange={(value) => {
                const clock_layout = value as "row" | "column";
                patch(instance, { clock_layout });
                void invoke("resize_widget", { id: instance, width: CLOCK_WIDTH[clock_layout] });
              }}
            />
          </div>
        );
      })}
    </section>
  );
}
