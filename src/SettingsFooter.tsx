import { SettingSwitch } from "./settings/SettingSwitch";

interface Props {
  arranging: boolean;
  dirty: boolean;
  onArrangeChange: (next: boolean) => void;
  onSave: () => void;
  onDiscard: () => void;
}

export function SettingsFooter({ arranging, dirty, onArrangeChange, onSave, onDiscard }: Props) {
  return (
    <footer className="settingsFooter">
      <SettingSwitch isSelected={arranging} onChange={onArrangeChange}>
        Arrange widgets
      </SettingSwitch>
      <div className="settingsFooterActions">
        <button type="button" className="settingsButton" disabled={!dirty} onClick={onDiscard}>
          Discard
        </button>
        <button type="button" className="settingsButton settingsButtonPrimary" disabled={!dirty} onClick={onSave}>
          Save
        </button>
      </div>
    </footer>
  );
}
