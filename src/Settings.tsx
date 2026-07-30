import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { Stats, StatsConfig, StatsProfile } from "./useStats";
import { Recovery } from "./Recovery";
import { Onboarding } from "./Onboarding";
import { SettingsNav } from "./SettingsNav";
import { SettingsSections } from "./SettingsSections";
import { SettingsFooter } from "./SettingsFooter";
import { SettingsClosePrompt } from "./SettingsClosePrompt";
import { isDirty, withoutSection, type MergedDraft } from "./settings-draft";

interface Props {
  stats: Stats | null;
}

export function Settings({ stats }: Props) {
  const [draft, setDraft] = useState<MergedDraft | null>(null);
  const [baseline, setBaseline] = useState<MergedDraft | null>(null);
  const [arranging, setArranging] = useState(false);
  const [prompting, setPrompting] = useState(false);
  const [recovery, setRecovery] = useState<unknown>(null);
  const [needsOnboarding, setNeedsOnboarding] = useState<boolean | null>(null);

  useEffect(() => {
    invoke<boolean>("needs_onboarding_cmd").then(setNeedsOnboarding);
    invoke("recovery_info").then(setRecovery);
  }, []);

  useEffect(() => {
    if (!stats || draft || needsOnboarding !== false || recovery) return;
    void Promise.all([
      invoke<boolean>("begin_settings_session"),
      invoke<boolean>("autostart_enabled"),
    ]).then(([armed, autostart]) => {
      const fresh = { config: stats.config, profile: stats.profile, autostart: autostart ?? false };
      setDraft(fresh);
      setBaseline(fresh);
      setArranging(Boolean(armed));
    });
  }, [stats, draft, needsOnboarding, recovery]);

  useEffect(() => {
    let unlisten = () => {};
    void listen<string>("widget-removed", ({ payload }) => {
      setDraft((current) => withoutSection(current, payload));
    }).then((cleanup) => { unlisten = cleanup; });
    return () => unlisten();
  }, []);

  const dirty = Boolean(draft && baseline && (isDirty(draft, baseline) || stats?.edit_touched));

  useEffect(() => {
    let unlisten = () => {};
    void listen("settings-close-requested", () => {
      if (dirty) setPrompting(true);
      else void getCurrentWindow().destroy();
    }).then((cleanup) => { unlisten = cleanup; });
    return () => unlisten();
  }, [dirty]);

  async function save() {
    if (!draft || !baseline) return;
    if (draft.autostart !== baseline.autostart) {
      await invoke("set_autostart", { enabled: draft.autostart });
    }
    await invoke("save_settings");
    setBaseline(draft);
  }

  async function discard() {
    if (!baseline) return;
    await invoke("discard_settings");
    setDraft(baseline);
  }

  if (needsOnboarding) return <Onboarding />;
  if (recovery) return <Recovery />;

  return (
    <main className="settingsShell" aria-label="Zokute settings">
      <header className="settingsHeader">
        <span className="settingsProduct">Zokute</span>
        <h1 className="settingsTitle">Settings</h1>
      </header>
      <SettingsNav />
      <div className="settingsPane" id="settings-pane">
        {draft ? (
          <SettingsSections
            stats={stats}
            draft={draft}
            onConfig={(next: StatsConfig) => {
              setDraft({ ...draft, config: next });
              void invoke("draft_config", { next });
            }}
            onProfile={(changes: Partial<StatsProfile>) => {
              const next = { ...draft.profile, ...changes };
              setDraft({ ...draft, profile: next });
              void invoke("draft_profile", { next });
            }}
            onAutostart={(autostart) => setDraft({ ...draft, autostart })}
          />
        ) : (
          <p className="settingsWaiting">Waiting for the first reading…</p>
        )}
      </div>
      <SettingsFooter
        arranging={arranging}
        dirty={dirty}
        onArrangeChange={(next) => {
          setArranging(next);
          void invoke("set_arrange", { enabled: next });
        }}
        onSave={() => void save()}
        onDiscard={() => void discard()}
      />
      {prompting ? (
        <SettingsClosePrompt
          onSave={() => void save().then(() => getCurrentWindow().destroy())}
          onDiscard={() => void discard().then(() => getCurrentWindow().destroy())}
          onKeepEditing={() => setPrompting(false)}
        />
      ) : null}
    </main>
  );
}
