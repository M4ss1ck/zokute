import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Stats, StatsConfig } from "./useStats";
import { ColorControl } from "./settings/Color";
import { OpacityControl } from "./settings/Opacity";
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
      <h1 className="settingsTitle">Zokute</h1>
      <div className="settingsBody">
        {draft ? (
          <>
            <OpacityControl
              id="settings-opacity"
              label="Background opacity"
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
            <OpacityControl
              id="settings-text-opacity"
              label="Text opacity"
              value={textPreview ?? draft.text_opacity ?? 1}
              onPreview={(value) => {
                setTextPreview(value);
                void invoke("preview_text_opacity", { value });
              }}
              onCommit={(text_opacity) => {
                setTextPreview(null);
                update({ ...draft, text_opacity });
              }}
            />
            <ColorControl
              id="settings-text-color"
              label="Text color"
              value={draft.text_color ?? "#292824"}
              onChange={(text_color) => update({ ...draft, text_color })}
            />
            <ColorControl
              id="settings-graph-color"
              label="Graph color"
              value={draft.graph_color ?? "#494137"}
              onChange={(graph_color) => update({ ...draft, graph_color })}
            />
            <ColorControl
              id="settings-icon-color"
              label="Icon color"
              value={draft.icon_color ?? "#c07100"}
              onChange={(icon_color) => update({ ...draft, icon_color })}
            />
            <div className="settingsRow">
              <label className="settingsRowLabel" htmlFor="settings-show-background">
                Show background
              </label>
              <input
                id="settings-show-background"
                type="checkbox"
                checked={draft.show_background ?? true}
                onChange={(event) => update({ ...draft, show_background: event.currentTarget.checked })}
              />
            </div>
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
