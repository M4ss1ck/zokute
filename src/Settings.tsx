import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Stats, StatsConfig } from "./useStats";
import { OpacityControl } from "./settings/Opacity";

interface Props {
  stats: Stats | null;
}

// The draft is seeded once and never re-seeded from the stream: two edits made
// inside the same second would otherwise both compose from the same stale
// reading, and the second would silently revert the first.
export function Settings({ stats }: Props) {
  const [draft, setDraft] = useState<StatsConfig | null>(null);
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
          <OpacityControl
            value={draft.opacity}
            onPreview={(value) => void invoke("preview_opacity", { value })}
            onCommit={(opacity) => update({ ...draft, opacity })}
          />
        ) : (
          <p className="settingsWaiting">Waiting for the first reading…</p>
        )}
      </div>
    </main>
  );
}
