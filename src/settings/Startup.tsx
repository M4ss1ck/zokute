import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SettingSwitch } from "./SettingSwitch";

export function StartupToggle() {
  const [enabled, setEnabled] = useState<boolean | null>(null);
  useEffect(() => {
    let active = true;
    void invoke<boolean>("autostart_enabled").then((value) => {
      if (active) setEnabled(value);
    });
    return () => {
      active = false;
    };
  }, []);
  function change(next: boolean) {
    setEnabled(next);
    void invoke("set_autostart", { enabled: next });
  }
  return (
    <section className="settingsCard" aria-labelledby="startup-title">
      <header className="settingsCardHeader">
        <h2 id="startup-title">Startup</h2>
        <p>Keep Zokute available when your desktop session begins.</p>
      </header>
      <div className="settingsCardBody">
        <SettingSwitch
          isSelected={enabled ?? false}
          isDisabled={enabled === null}
          onChange={change}
        >
          Start with the session
        </SettingSwitch>
      </div>
    </section>
  );
}
