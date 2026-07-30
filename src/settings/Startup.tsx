import { SettingSwitch } from "./SettingSwitch";

interface Props {
  enabled: boolean;
  onChange: (next: boolean) => void;
}

export function StartupToggle({ enabled, onChange }: Props) {
  return (
    <section className="settingsCard" aria-labelledby="startup-title">
      <header className="settingsCardHeader">
        <h2 id="startup-title">Startup</h2>
        <p>Keep Zokute available when your desktop session begins.</p>
      </header>
      <div className="settingsCardBody">
        <SettingSwitch isSelected={enabled} onChange={onChange}>
          Start with the session
        </SettingSwitch>
      </div>
    </section>
  );
}
