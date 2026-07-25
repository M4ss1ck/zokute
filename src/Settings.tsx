import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Stats, StatsConfig } from "./useStats";
import { Appearance } from "./settings/Appearance";
import { SectionToggles } from "./settings/Sections";
import { FieldToggles } from "./settings/Fields";
import { DiskPreferences } from "./settings/Disks";
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
            <DiskPreferences detected={stats?.disks ?? []} config={draft} onChange={update} />
            <StartupToggle />
          </>
        ) : (
          <p className="settingsWaiting">Waiting for the first reading…</p>
        )}
      </div>
    </main>
  );
}
