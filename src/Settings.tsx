import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Stats, StatsConfig } from "./useStats";
import { OpacityControl } from "./settings/Opacity";
import { SectionToggles } from "./settings/Sections";
import { FieldToggles } from "./settings/Fields";
import { DiskPreferences } from "./settings/Disks";

interface Props {
  stats: Stats | null;
}

// The draft is seeded once and never re-seeded from the stream: two edits made
// inside the same second would otherwise both compose from the same stale
// reading, and the second would silently revert the first.
export function Settings({ stats }: Props) {
  const [draft, setDraft] = useState<StatsConfig | null>(null);
  const [preview, setPreview] = useState<number | null>(null);
  useEffect(() => {
    if (stats && !draft) setDraft(stats.config);
  }, [stats, draft]);
  function update(next: StatsConfig) {
    setDraft(next);
    void invoke("update_config", { next });
  }
  return (
    <main className="settings" aria-label="Zokute settings">
      <h1 className="settingsTitle">Zokute</h1>
      <div className="settingsBody">
        {draft ? (
          <>
            <OpacityControl
              value={preview ?? draft.opacity}
              onPreview={(value) => {
                setPreview(value);
                void invoke("preview_opacity", { value });
              }}
              onCommit={(opacity) => {
                setPreview(null);
                update({ ...draft, opacity });
              }}
            />
            <SectionToggles
              config={draft}
              monitorCount={Math.max(...draft.sections.map((section) => section.monitor), 0) + 2}
              onChange={update}
            />
            <FieldToggles available={stats?.system_fields ?? []} config={draft} onChange={update} />
            <DiskPreferences detected={stats?.disks ?? []} config={draft} onChange={update} />
          </>
        ) : (
          <p className="settingsWaiting">Waiting for the first reading…</p>
        )}
      </div>
    </main>
  );
}
