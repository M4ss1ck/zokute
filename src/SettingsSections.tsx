import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Stats, StatsConfig, StatsProfile } from "./useStats";
import type { MergedDraft } from "./settings-draft";
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

interface Props {
  stats: Stats | null;
  draft: MergedDraft;
  onConfig: (next: StatsConfig) => void;
  onProfile: (changes: Partial<StatsProfile>) => void;
  onAutostart: (next: boolean) => void;
}

export function SettingsSections({ stats, draft, onConfig, onProfile, onAutostart }: Props) {
  const [preview, setPreview] = useState<number | null>(null);
  const [textPreview, setTextPreview] = useState<number | null>(null);
  const merged = { ...draft.config, ...draft.profile };
  const sections = (next: { sections: StatsProfile["sections"] }) => onProfile({ sections: next.sections });

  return (
    <>
      <div className="settingsAnchor" id="appearance">
        <Appearance
          config={draft.config}
          backgroundPreview={preview}
          textPreview={textPreview}
          onBackgroundPreview={(value) => { setPreview(value); void invoke("preview_opacity", { value }); }}
          onBackgroundCommit={(opacity) => { setPreview(null); onConfig({ ...draft.config, opacity }); }}
          onTextPreview={(value) => { setTextPreview(value); void invoke("preview_text_opacity", { value }); }}
          onTextCommit={(text_opacity) => { setTextPreview(null); onConfig({ ...draft.config, text_opacity }); }}
          onChange={onConfig}
        />
      </div>
      <div className="settingsAnchor" id="widgets"><SectionToggles config={merged} onChange={sections} /></div>
      <div className="settingsAnchor" id="panels"><PanelPreferences config={merged} onChange={sections} /></div>
      <div className="settingsAnchor" id="plugins"><PluginPreferences config={merged} onChange={sections} /></div>
      <div className="settingsAnchor" id="startup">
        <StartupToggle enabled={draft.autostart} onChange={onAutostart} />
      </div>
      <div className="settingsAnchor" id="fields">
        <FieldToggles available={stats?.system_fields ?? []} config={merged} onChange={(next) => onProfile({ system_fields: next.system_fields })} />
      </div>
      <div className="settingsAnchor" id="clock"><ClockPreferences config={merged} onChange={sections} /></div>
      <div className="settingsAnchor" id="date"><DatePreferences config={merged} onChange={sections} /></div>
      <div className="settingsAnchor" id="visualizer"><VisualizerPreferences config={merged} onChange={sections} /></div>
      <div className="settingsAnchor" id="cpu">
        <CpuPreferences config={merged} onChange={(next) => onProfile({ show_cpu_cores: next.show_cpu_cores })} />
      </div>
      <div className="settingsAnchor" id="memory"><MemoryPreferences config={merged} onChange={() => {}} /></div>
      <div className="settingsAnchor" id="disks">
        <DiskPreferences detected={stats?.disks ?? []} config={merged} onChange={(next) => onProfile({ disks: next.disks })} />
      </div>
      <div className="settingsAnchor" id="network"><NetworkPreferences config={merged} onChange={() => {}} /></div>
    </>
  );
}
