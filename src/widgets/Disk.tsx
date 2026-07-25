import { IconDatabase } from "@tabler/icons-react";
import { Bar } from "../viz/Bar";
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

function formatTemperature(celsius: number) {
  return `${celsius.toFixed(1)}°C`;
}

export function DiskWidget({ stats }: Props) {
  return (
    <section className="panel">
      <header className="panelHeader">
        <span className="panelTitleGroup">
          <IconDatabase className="panelIcon" />
          <span className="panelTitle">Disk</span>
        </span>
        <span className="panelValue">{stats.disks.length}</span>
      </header>
      <div className="diskList">
        {stats.disks.map((disk) => {
          const percent = clampPercent(disk.used_bytes, disk.total_bytes);
          const label = disk.display_label ?? disk.name;
          return (
            <div className="diskRow" key={disk.id}>
              <div className="diskHeader">
                <div className="diskIdentity">
                  <span className="diskName">{label}</span>
                  <span className="diskMount">{disk.mount}</span>
                </div>
                <span className="diskValue">
                  {formatBytes(disk.used_bytes)} / {formatBytes(disk.total_bytes)}
                </span>
              </div>
              {disk.temperature_celsius !== null ? (
                <span className="diskTemperature">{formatTemperature(disk.temperature_celsius)}</span>
              ) : null}
              <Bar percent={percent} />
            </div>
          );
        })}
      </div>
    </section>
  );
}
