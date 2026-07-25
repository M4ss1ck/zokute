import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Stats, StatsConfig } from "./useStats";
import { Appearance } from "./settings/Appearance";
import { CpuPreferences } from "./settings/Cpu";
import { SectionToggles } from "./settings/Sections";
import { FieldToggles } from "./settings/Fields";
import { DiskPreferences } from "./settings/Disks";
import { MemoryPreferences } from "./settings/Memory";
import { NetworkPreferences } from "./settings/Network";
import { StartupToggle } from "./settings/Startup";

interface Props {
  stats: Stats | null;
}

// The draft is seeded once and never re-seeded from the stream: two edits made
// inside the same second would otherwise both compose from the same stale
// reading, and the second would silently revert the first.
export function Settings({ stats }: Props) {
  const [draft, setDraft] = useState<StatsConfig | null>(null);
  const [preview, setPreview] = useState<number | null>(null);
  const [textPreview, setTextPreview] = useState<number | null>(null);
  useEffect(() => {
    if (stats && !draft) setDraft(stats.config);
  }, [stats, draft]);
  useEffect(() => {
    let active = true;
    let unlisten = () => {};
    void listen<string>("widget-removed", ({ payload }) => {
      if (!active) return;
      setDraft((current) => current ? {
        ...current,
        sections: current.sections.filter((section) => (section.instance ?? section.id) !== payload),
      } : current);
    }).then((cleanup) => {
      if (active) unlisten = cleanup;
      else void cleanup();
    });
    return () => {
      active = false;
      unlisten();
    };
  }, []);
  function update(next: StatsConfig) {
    setDraft(next);
    void invoke("update_config", { next });
  }
  return (
    <main className="settings" aria-label="Zokute settings">
      <header className="settingsHeader">
        <span className="settingsProduct">Zokute</span>
        <h1 className="settingsTitle">Settings</h1>
        <p>Shape the overlay around the way you work.</p>
      </header>
      <div className="settingsBody">
        {draft ? (
          <>
            <Appearance
              config={draft}
              backgroundPreview={preview}
              textPreview={textPreview}
              onBackgroundPreview={(value) => {
                setPreview(value);
                void invoke("preview_opacity", { value });
              }}
              onBackgroundCommit={(opacity) => {
                setPreview(null);
                update({ ...draft, opacity });
              }}
              onTextPreview={(value) => {
                setTextPreview(value);
                void invoke("preview_text_opacity", { value });
              }}
              onTextCommit={(text_opacity) => {
                setTextPreview(null);
                update({ ...draft, text_opacity });
              }}
              onChange={update}
            />
            <SectionToggles
              config={draft}
              onChange={update}
            />
            <FieldToggles available={stats?.system_fields ?? []} config={draft} onChange={update} />
            <CpuPreferences config={draft} onChange={update} />
            <MemoryPreferences config={draft} onChange={update} />
            <DiskPreferences detected={stats?.disks ?? []} config={draft} onChange={update} />
            <NetworkPreferences config={draft} onChange={update} />
            <StartupToggle />
          </>
        ) : (
          <p className="settingsWaiting">Waiting for the first reading…</p>
        )}
      </div>
    </main>
  );
}
