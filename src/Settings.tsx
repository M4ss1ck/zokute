import type { Stats } from "./useStats";

interface Props {
  stats: Stats | null;
}

export function Settings({ stats }: Props) {
  return (
    <main className="settings" aria-label="Zokute settings">
      <h1 className="settingsTitle">Zokute</h1>
      <div className="settingsBody">
        {stats ? null : <p className="settingsWaiting">Waiting for the first reading…</p>}
      </div>
    </main>
  );
}
