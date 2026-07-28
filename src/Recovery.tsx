import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface RecoveryInfo {
  message: string;
  config_path: string;
  backups_dir: string;
}

export function Recovery() {
  const [info, setInfo] = useState<RecoveryInfo | null>(null);
  const [busy, setBusy] = useState<string | null>(null);

  useEffect(() => {
    invoke<RecoveryInfo | null>("recovery_info").then(setInfo);
  }, []);

  if (!info) return null;

  async function act(action: string) {
    setBusy(action);
    try {
      const result = await invoke<string>("recovery_action", { action });
      if (result === "valid" || result === "defaults_in_memory") {
        setInfo(null as unknown as RecoveryInfo);
      } else if (result === "invalid") {
        setInfo(null as unknown as RecoveryInfo);
        await invoke<RecoveryInfo | null>("recovery_info").then(setInfo);
      } else {
        setInfo(null as unknown as RecoveryInfo);
      }
    } catch (e) {
      console.error("recovery action failed:", e);
    }
    setBusy(null);
  }

  async function open(path: string) {
    try {
      await invoke("open_path", { path });
    } catch (e) {
      console.error("open failed:", e);
    }
  }

  return (
    <main className="recovery" aria-label="Zokute recovery">
      <header className="settingsHeader">
        <span className="settingsProduct">Zokute</span>
        <h1 className="settingsTitle">Configuration Issue</h1>
      </header>
      <div className="settingsBody">
        <div className="recoveryError">
          <p>{info.message}</p>
          <p className="recoveryPath">Config: <code>{info.config_path}</code></p>
          <p className="recoveryPath">Backups: <code>{info.backups_dir}</code></p>
        </div>
        <div className="recoveryActions">
          <button className="recoveryButtonSecondary" onClick={() => open(info.config_path)} disabled={busy !== null}>
            Open Config File
          </button>
          <button className="recoveryButtonSecondary" onClick={() => open(info.backups_dir)} disabled={busy !== null}>
            Open Backups Directory
          </button>
          <button onClick={() => act("restore_backup")} disabled={busy !== null}>
            {busy === "restore_backup" ? "Restoring…" : "Restore Previous Backup"}
          </button>
          <button onClick={() => act("revalidate")} disabled={busy !== null}>
            {busy === "revalidate" ? "Validating…" : "Validate Again"}
          </button>
          <button onClick={() => act("use_in_memory_defaults")} disabled={busy !== null}>
            {busy === "use_in_memory_defaults" ? "Loading…" : "Use Defaults (In-Memory Only)"}
          </button>
          <button onClick={() => act("use_defaults")} disabled={busy !== null}>
            {busy === "use_defaults" ? "Creating…" : "Overwrite with Default Configuration"}
          </button>
        </div>
      </div>
    </main>
  );
}