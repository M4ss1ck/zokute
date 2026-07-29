import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Stats, StatsConfig, StatsProfile } from "./useStats";
import { Appearance } from "./settings/Appearance";
import { CpuPreferences } from "./settings/Cpu";
import { SectionToggles } from "./settings/Sections";
import { FieldToggles } from "./settings/Fields";
import { DiskPreferences } from "./settings/Disks";
import { MemoryPreferences } from "./settings/Memory";
import { NetworkPreferences } from "./settings/Network";
import { StartupToggle } from "./settings/Startup";
import { VisualizerPreferences } from "./settings/Visualizer";
import { ClockPreferences } from "./settings/Clock";
import { DatePreferences } from "./settings/Date";
import { PanelPreferences } from "./settings/Panels";
import { PluginPreferences } from "./settings/Plugins";
import { Recovery } from "./Recovery";
import { Onboarding } from "./Onboarding";
import { withoutSection, type MergedDraft } from "./settings-draft";

interface Props {
  stats: Stats | null;
}

export function Settings({ stats }: Props) {
  const [draft, setDraft] = useState<MergedDraft | null>(null);
  const [preview, setPreview] = useState<number | null>(null);
  const [textPreview, setTextPreview] = useState<number | null>(null);
  const [recovery, setRecovery] = useState<unknown>(null);
  const [needsOnboarding, setNeedsOnboarding] = useState<boolean | null>(null);

  useEffect(() => {
    invoke<boolean>("needs_onboarding_cmd").then(setNeedsOnboarding);
  }, []);

  useEffect(() => {
    if (stats && !draft) {
      setDraft({
        config: stats.config,
        profile: stats.profile,
      });
    }
  }, [stats, draft]);

  useEffect(() => {
    let active = true;
    invoke("recovery_info").then((info) => {
      if (active) setRecovery(info);
    });
    return () => { active = false; };
  }, []);

  useEffect(() => {
    let active = true;
    let unlisten = () => {};
    void listen<string>("widget-removed", ({ payload }) => {
      if (!active) return;
      setDraft((current) => withoutSection(current, payload));
    }).then((cleanup) => {
      if (active) unlisten = cleanup;
      else void cleanup();
    });
    return () => {
      active = false;
      unlisten();
    };
  }, []);

  function updateConfig(next: StatsConfig) {
    if (!draft) return;
    setDraft({ ...draft, config: next });
    void invoke("update_config", { next });
  }

  function updateProfile(next: StatsProfile) {
    if (!draft) return;
    setDraft({ ...draft, profile: next });
    void invoke("update_profile", { next });
  }

  function patchProfile(changes: Partial<StatsProfile>) {
    updateProfile({ ...draft!.profile, ...changes });
  }

  if (needsOnboarding) return <Onboarding />;
  if (recovery) return <Recovery />;

  const mergedConfig = draft ? { ...draft.config, ...draft.profile } : null;

  return (
    <main className="settings" aria-label="Zokute settings">
      <header className="settingsHeader">
        <span className="settingsProduct">Zokute</span>
        <h1 className="settingsTitle">Settings</h1>
        <p>Shape the overlay around the way you work.</p>
      </header>
      <div className="settingsBody">
        {draft && mergedConfig ? (
          <>
            <Appearance
              config={draft.config}
              backgroundPreview={preview}
              textPreview={textPreview}
              onBackgroundPreview={(value) => {
                setPreview(value);
                void invoke("preview_opacity", { value });
              }}
              onBackgroundCommit={(opacity) => {
                setPreview(null);
                updateConfig({ ...draft.config, opacity });
              }}
              onTextPreview={(value) => {
                setTextPreview(value);
                void invoke("preview_text_opacity", { value });
              }}
              onTextCommit={(text_opacity) => {
                setTextPreview(null);
                updateConfig({ ...draft.config, text_opacity });
              }}
              onChange={updateConfig}
            />
            <SectionToggles config={mergedConfig} onChange={(next) => patchProfile({ sections: next.sections })} />
            <VisualizerPreferences config={mergedConfig} onChange={(next) => patchProfile({ sections: next.sections })} />
            <ClockPreferences config={mergedConfig} onChange={(next) => patchProfile({ sections: next.sections })} />
            <DatePreferences config={mergedConfig} onChange={(next) => patchProfile({ sections: next.sections })} />
            <PanelPreferences config={mergedConfig} onChange={(next) => patchProfile({ sections: next.sections })} />
            <PluginPreferences config={mergedConfig} onChange={(next) => patchProfile({ sections: next.sections })} />
            <FieldToggles available={stats?.system_fields ?? []} config={mergedConfig} onChange={(next) => patchProfile({ system_fields: next.system_fields })} />
            <CpuPreferences config={mergedConfig} onChange={(next) => patchProfile({ show_cpu_cores: next.show_cpu_cores })} />
            <MemoryPreferences config={mergedConfig} onChange={() => {}} />
            <DiskPreferences detected={stats?.disks ?? []} config={mergedConfig} onChange={(next) => patchProfile({ disks: next.disks })} />
            <NetworkPreferences config={mergedConfig} onChange={() => {}} />
            <StartupToggle />
          </>
        ) : (
          <p className="settingsWaiting">Waiting for the first reading…</p>
        )}
      </div>
    </main>
  );
}
