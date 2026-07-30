import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import type { StatsConfig, StatsProfile } from "./useStats";

interface Props {
  onReload: (config: StatsConfig, profile: StatsProfile) => void;
  onBusyChange: (busy: boolean) => void;
}

interface AcceptedExternal {
  config: StatsConfig;
  profile: StatsProfile;
}

export function SettingsExternalChange({ onReload, onBusyChange }: Props) {
  const [visible, setVisible] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let active = true;
    const showNotice = () => {
      if (active) setVisible(true);
    };
    const subscriptions = Promise.all([
      listen("external-config-changed", showNotice),
      listen("external-profile-changed", showNotice),
    ]);

    return () => {
      active = false;
      void subscriptions.then((unlisteners) => unlisteners.forEach((unlisten) => unlisten()));
    };
  }, []);

  if (!visible) return null;

  const resolve = async (command: "accept_external_config" | "dismiss_external_config") => {
    setError(null);
    onBusyChange(true);
    try {
      const accepted = await invoke<AcceptedExternal | undefined>(command);
      setVisible(false);
      if (accepted) onReload(accepted.config, accepted.profile);
    } catch (reason: unknown) {
      setError(String(reason));
    } finally {
      onBusyChange(false);
    }
  };

  return (
    <div className="settingsExternalChange" role="status">
      <span>Settings changed on disk.</span>
      {error ? <span role="alert">{error}</span> : null}
      <div className="settingsExternalChangeActions">
        <button type="button" className="settingsButton" onClick={() => void resolve("dismiss_external_config")}>
          Keep my draft
        </button>
        <button type="button" className="settingsButton settingsButtonPrimary" onClick={() => void resolve("accept_external_config")}>
          Reload from disk
        </button>
      </div>
    </div>
  );
}
