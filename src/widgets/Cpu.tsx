import type { Stats } from "../useStats";

interface Props {
  stats: Stats;
}

function clampPercent(value: number) {
  return Number.isFinite(value) ? Math.max(0, Math.min(100, value)) : 0;
}

export function CpuWidget({ stats }: Props) {
  const aggregate = clampPercent(stats.cpu.aggregate_percent);
  return (
    <section className="panel">
      <header className="panelHeader">
        <span className="panelTitle">CPU</span>
        <span className="panelValue">{aggregate.toFixed(1)}%</span>
      </header>
      <div className="metric">
        <span className="metricLabel">Aggregate</span>
        <span className="metricValue">{aggregate.toFixed(1)}%</span>
      </div>
      <div className="barTrack" aria-hidden="true">
        <div className="barFill" style={{ width: `${aggregate}%` }} />
      </div>
      <div className="coreGrid">
        {stats.cpu.core_percents.map((percent, index) => {
          const value = clampPercent(percent);
          return (
            <div className="coreCell" key={index}>
              <span className="coreLabel">C{index + 1}</span>
              <div className="barTrack" aria-hidden="true">
                <div className="barFill" style={{ width: `${value}%` }} />
              </div>
            </div>
          );
        })}
      </div>
    </section>
  );
}
