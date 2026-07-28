import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

const THEMES = [
  { id: "light", label: "Atelier Light" },
  { id: "dark", label: "Atelier Dark" },
  { id: "system", label: "System" },
];

const PRESETS = [
  {
    id: "minimal",
    label: "Minimal",
    desc: "Clock and date only — clean and simple",
  },
  {
    id: "system_monitor",
    label: "System Monitor",
    desc: "Clock, date, CPU, memory, disk, and network",
  },
  {
    id: "blank",
    label: "Blank",
    desc: "No widgets — start from scratch in Settings",
  },
];

export function Onboarding() {
  const [theme, setTheme] = useState("light");
  const [preset, setPreset] = useState("system_monitor");
  const [autostart, setAutostart] = useState(true);
  const [done, setDone] = useState(false);
  const [configPath, setConfigPath] = useState("");
  const [busy, setBusy] = useState(false);

  async function finish() {
    setBusy(true);
    try {
      await invoke("apply_onboarding", {
        choice: { theme, preset, autostart },
      });
      setDone(true);
      const info: { config_path: string } | null = await invoke("recovery_info");
      if (info) setConfigPath(info.config_path);
    } catch (e) {
      console.error("onboarding failed:", e);
    }
    setBusy(false);
  }

  async function openConfig() {
    if (configPath) {
      await invoke("open_path", { path: configPath });
    }
  }

  async function openLayoutEditor() {
    await invoke("enter_edit_layout");
  }

  if (done) {
    return (
      <main className="settings" aria-label="Zokute onboarding complete">
        <header className="settingsHeader">
          <span className="settingsProduct">Zokute</span>
          <h1 className="settingsTitle">Ready to Go</h1>
          <p>Your configuration has been saved.</p>
        </header>
        <div className="settingsBody">
          <p>Your config is at:</p>
          {configPath && (
            <button className="recoveryButtonSecondary" onClick={openConfig}>
              <code>{configPath}</code>
            </button>
          )}
          <div style={{ marginTop: "1rem" }}>
            <button onClick={openLayoutEditor}>
              Edit Layout
            </button>
          </div>
        </div>
      </main>
    );
  }

  return (
    <main className="settings" aria-label="Zokute onboarding">
      <header className="settingsHeader">
        <span className="settingsProduct">Zokute</span>
        <h1 className="settingsTitle">Welcome</h1>
        <p>Choose your starting setup — you can change everything later.</p>
      </header>
      <div className="settingsBody">
        <section>
          <h2>Theme</h2>
          <div className="onboardingOptions">
            {THEMES.map((t) => (
              <button
                key={t.id}
                className={theme === t.id ? "onboardingSelected" : ""}
                onClick={() => setTheme(t.id)}
                disabled={busy}
              >
                {t.label}
              </button>
            ))}
          </div>
        </section>
        <section>
          <h2>Preset</h2>
          <div className="onboardingOptions">
            {PRESETS.map((p) => (
              <button
                key={p.id}
                className={preset === p.id ? "onboardingSelected" : ""}
                onClick={() => setPreset(p.id)}
                disabled={busy}
              >
                <strong>{p.label}</strong>
                <br />
                <small>{p.desc}</small>
              </button>
            ))}
          </div>
        </section>
        <section>
          <h2>Autostart</h2>
          <label className="onboardingToggle">
            <input
              type="checkbox"
              checked={autostart}
              onChange={(e) => setAutostart(e.target.checked)}
              disabled={busy}
            />
            Start Zokute automatically after login
          </label>
        </section>
        <section>
          <button onClick={finish} disabled={busy}>
            {busy ? "Saving…" : "Finish Setup"}
          </button>
        </section>
      </div>
    </main>
  );
}
