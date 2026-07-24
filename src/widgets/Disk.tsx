import type { Stats } from "../useStats";

interface Props {
  stats: Stats;
}

function clampPercent(used: number, total: number) {
  if (!Number.isFinite(used) || !Number.isFinite(total) || total <= 0) return 0;
  return Math.max(0, Math.min(100, (used / total) * 100));
}

function formatBytes(bytes: number) {
  const units = ["B", "KB", "MB", "GB", "TB"];
  let size = Math.max(0, bytes);
  let index = 0;
  while (size >= 1024 && index < units.length - 1) {
    size /= 1024;
    index += 1;
  }
  return `${size.toFixed(size >= 10 || index === 0 ? 0 : 1)} ${units[index]}`;
}

export function DiskWidget({ stats }: Props) {
  return (
    <section className="panel">
      <header className="panelHeader">
        <span className="panelTitle">Disk</span>
        <span className="panelValue">{stats.disks.length}</span>
      </header>
      <div className="diskList">
        {stats.disks.map((disk) => {
          const percent = clampPercent(disk.used_bytes, disk.total_bytes);
          return (
            <div className="diskRow" key={`${disk.name}-${disk.mount}`}>
              <div className="diskHeader">
                <div className="diskIdentity">
                  <span className="diskName">{disk.name}</span>
                  <span className="diskMount">{disk.mount}</span>
                </div>
                <span className="diskValue">
                  {formatBytes(disk.used_bytes)} / {formatBytes(disk.total_bytes)}
                </span>
              </div>
              <div className="barTrack" aria-hidden="true">
                <div className="barFill" style={{ width: `${percent}%` }} />
              </div>
            </div>
          );
        })}
      </div>
    </section>
  );
}
