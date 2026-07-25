import type { CSSProperties } from "react";

const PRESETS = [
  { label: "Ink", color: "#292824" },
  { label: "Black", color: "#000000" },
  { label: "White", color: "#ffffff" },
  { label: "Cream", color: "#f7f4ea" },
];

interface Props {
  value: string;
  onChange: (value: string) => void;
}

type PresetStyle = CSSProperties & { "--preset-color": string };

export function ColorControl({ value, onChange }: Props) {
  return (
    <div className="settingsRow">
      <label className="settingsRowLabel" htmlFor="settings-text-color">
        Text color
      </label>
      <div className="settingsColorControls">
        <input
          className="settingsColorInput"
          id="settings-text-color"
          type="color"
          value={value}
          onChange={(event) => onChange(event.currentTarget.value)}
        />
        <div className="settingsColorPresets" aria-label="Text color presets">
          {PRESETS.map(({ label, color }) => (
            <button
              className="settingsColorPreset"
              type="button"
              key={color}
              aria-label={`Use ${label.toLowerCase()} text`}
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
