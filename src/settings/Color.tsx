import {
  ColorSwatch,
  ColorSwatchPicker,
  ColorSwatchPickerItem,
} from "react-aria-components";

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
  label: string;
  value: string;
  onChange: (value: string) => void;
}

export function ColorControl({ label, value, onChange }: Props) {
  return (
    <div className="settingsColor">
      <span className="settingsRowLabel">{label}</span>
      <div className="settingsColorControls">
        <input
          className="settingsColorInput"
          type="color"
          aria-label={label}
          value={value}
          onChange={(event) => onChange(event.currentTarget.value)}
        />
        <ColorSwatchPicker
          className="settingsColorPresets"
          aria-label={`${label} presets`}
          value={value}
          onChange={(color) => onChange(color.toString("hex").toLowerCase())}
        >
          {PRESETS.map(({ label: presetLabel, color }) => (
            <ColorSwatchPickerItem
              className="settingsColorPreset"
              key={color}
              color={color}
              aria-label={`Use ${presetLabel.toLowerCase()} ${label.toLowerCase()}`}
            >
              <ColorSwatch />
            </ColorSwatchPickerItem>
          ))}
        </ColorSwatchPicker>
      </div>
    </div>
  );
}
