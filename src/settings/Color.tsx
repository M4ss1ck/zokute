import type { CSSProperties } from "react";

const PRESETS = [
  { label: "Ink", color: "#292824" },
  { label: "Graphite", color: "#494137" },
  { label: "Amber", color: "#c07100" },
  { label: "Blue", color: "#2563eb" },
  { label: "Green", color: "#16a34a" },
  { label: "Black", color: "#000000" },
  { label: "White", color: "#ffffff" },
  { label: "Cream", color: "#f7f4ea" },
];

interface Props {
  id: string;
  label: string;
  value: string;
  onChange: (value: string) => void;
}

type PresetStyle = CSSProperties & { "--preset-color": string };

export function ColorControl({ id, label, value, onChange }: Props) {
  return (
    <div className="settingsRow">
      <label className="settingsRowLabel" htmlFor={id}>
        {label}
      </label>
      <div className="settingsColorControls">
        <input
          className="settingsColorInput"
          id={id}
          type="color"
          value={value}
          onChange={(event) => onChange(event.currentTarget.value)}
        />
        <div className="settingsColorPresets" aria-label={`${label} presets`}>
          {PRESETS.map(({ label: presetLabel, color }) => (
            <button
              className="settingsColorPreset"
              type="button"
              key={color}
              aria-label={`Use ${presetLabel.toLowerCase()} ${label.toLowerCase()}`}
              aria-pressed={value.toLowerCase() === color}
              style={{ "--preset-color": color } as PresetStyle}
              onClick={() => onChange(color)}
            />
          ))}
        </div>
      </div>
    </div>
  );
}
