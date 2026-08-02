import type { MergedConfig, SectionConfig } from "../useStats";
import { instanceTitle } from "../instance-title";
import { ColorControl } from "./Color";
import { SettingSwitch } from "./SettingSwitch";
import { SettingsToggleGroup } from "./ToggleGroup";

const FORMATS = [
  { id: "long", label: "Long" },
  { id: "short", label: "Short" },
  { id: "numeric", label: "Numeric" },
];

interface Props {
  config: MergedConfig;
  onChange: (next: MergedConfig) => void;
}

export function DatePreferences({ config, onChange }: Props) {
  const dates = config.sections.filter((section) => section.enabled && section.id === "date");
  if (dates.length === 0) return null;
  function patch(instance: string, changes: Partial<SectionConfig>) {
    onChange({
      ...config,
      sections: config.sections.map((section) =>
        (section.instance ?? section.id) === instance ? { ...section, ...changes } : section,
      ),
    });
  }
  return (
    <section className="settingsCard" aria-labelledby="dates-title">
      <header className="settingsCardHeader">
        <h2 id="dates-title">Dates</h2>
        <p>Choose how each date appears.</p>
      </header>
      {dates.map((section) => {
        const instance = section.instance ?? section.id;
        return (
          <div className="settingsCardBody settingsInstance" key={instance}>
            <h3 className="settingsInstanceHeader" id={`instance-${instance}`}>{instanceTitle(instance)}</h3>
            <SettingSwitch
              isSelected={section.date_weekday ?? true}
              onChange={(date_weekday) => patch(instance, { date_weekday })}
            >
              Show weekday
            </SettingSwitch>
            <SettingsToggleGroup
              label="Format"
              options={FORMATS}
              value={section.date_format ?? "long"}
              onChange={(date_format) => patch(instance, { date_format: date_format as SectionConfig["date_format"] })}
            />
            <ColorControl
              label="Color"
              value={section.date_color ?? config.text_color ?? "#292824"}
              onChange={(date_color) => patch(instance, { date_color })}
            />
          </div>
        );
      })}
    </section>
  );
}
