import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function StartupToggle() {
  const [enabled, setEnabled] = useState(false);
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
    <fieldset className="settingsGroup">
      <legend className="settingsGroupTitle">Startup</legend>
      <div className="settingsRow">
        <label className="settingsRowLabel" htmlFor="settings-autostart">
          Start with the session
        </label>
        <input
          id="settings-autostart"
          type="checkbox"
          checked={enabled}
          onChange={(event) => change(event.currentTarget.checked)}
        />
      </div>
    </fieldset>
  );
}
