import { ToggleButton, ToggleButtonGroup } from "react-aria-components";

interface Props {
  label: string;
  value: string;
  options: ReadonlyArray<{ id: string; label: string }>;
  onChange: (value: string) => void;
}

export function SettingsToggleGroup({ label, value, options, onChange }: Props) {
  return (
    <div className="settingsToggleRow">
      <span className="settingsRowLabel">{label}</span>
      <ToggleButtonGroup
        className="settingsToggleGroup"
        aria-label={label}
        disallowEmptySelection
        selectionMode="single"
        selectedKeys={[value]}
        onSelectionChange={(keys) => {
          const [selected] = keys;
          // `disallowEmptySelection` still fires with an empty set when the
          // selected button is re-pressed; that is a no-op, not a change.
          if (typeof selected === "string") onChange(selected);
        }}
      >
        {options.map((option) => (
          <ToggleButton className="settingsToggle" id={option.id} key={option.id}>
            {option.label}
          </ToggleButton>
        ))}
      </ToggleButtonGroup>
    </div>
  );
}
