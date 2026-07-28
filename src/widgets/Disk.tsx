import { IconDatabase } from "@tabler/icons-react";
import { Bar } from "../viz/Bar";
import { formatBytes, formatTemperature } from "../format";
import type { Stats } from "../useStats";

interface Props {
  stats: Stats;
}

function clampPercent(used: number, total: number) {
  if (!Number.isFinite(used) || !Number.isFinite(total) || total <= 0) return 0;
  return Math.max(0, Math.min(100, (used / total) * 100));
}

export function DiskWidget({ stats }: Props) {
  const showHeader = stats.config.sections.find((section) => section.id === "disk")?.show_header ?? true;
  const byteMode = (stats.config.byte_format ?? "binary") as "binary" | "decimal";
  const tempUnit = (stats.config.temperature_unit ?? "celsius") as "celsius" | "fahrenheit";
  return (
    <section className="panel">
      {showHeader ? (
        <header className="panelHeader">
          <span className="panelTitleGroup">
            <IconDatabase className="panelIcon" />
            <span className="panelTitle">Disk</span>
          </span>
          <span className="panelValue">{stats.disks.length}</span>
        </header>
      ) : null}
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
                  {formatBytes(disk.used_bytes, byteMode)} / {formatBytes(disk.total_bytes, byteMode)}
                </span>
              </div>
              {disk.temperature_celsius !== null ? (
                <span className="diskTemperature">{formatTemperature(disk.temperature_celsius, tempUnit)}</span>
              ) : null}
              <Bar percent={percent} />
            </div>
          );
        })}
      </div>
    </section>
  );
}
