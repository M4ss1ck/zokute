import { SettingSwitch } from "./settings/SettingSwitch";

interface Props {
  arranging: boolean;
  dirty: boolean;
  error: string | null;
  onArrangeChange: (next: boolean) => void;
  onSave: () => void;
  onDiscard: () => void;
}

export function SettingsFooter({ arranging, dirty, error, onArrangeChange, onSave, onDiscard }: Props) {
  return (
    <footer className="settingsFooter">
      <SettingSwitch isSelected={arranging} onChange={onArrangeChange}>
        Arrange widgets
      </SettingSwitch>
      {error ? <span className="settingsSaveError" role="alert">{error}</span> : null}
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
