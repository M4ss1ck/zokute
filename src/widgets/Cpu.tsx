import { IconCpu } from "@tabler/icons-react";
import { Bar } from "../viz/Bar";
import { Sparkline } from "../viz/Sparkline";
import type { Stats, StatsHistory } from "../useStats";

interface Props {
  stats: Stats;
  history: StatsHistory;
}

function clampPercent(value: number) {
  return Number.isFinite(value) ? Math.max(0, Math.min(100, value)) : 0;
}

export function CpuWidget({ stats, history }: Props) {
  const aggregate = clampPercent(stats.cpu.aggregate_percent);
  return (
    <section className="panel">
      <header className="panelHeader">
        <span className="panelTitleGroup">
          <IconCpu className="panelIcon" />
          <span className="panelTitle">CPU</span>
        </span>
        <span className="panelValue">{aggregate.toFixed(1)}%</span>
      </header>
      <Sparkline values={history.cpuAggregate} min={0} max={100} />
      <div className="coreGrid">
        {stats.cpu.core_percents.map((percent, index) => (
          <div className="coreCell" key={index}>
            <span className="coreLabel">C{index + 1}</span>
            <Bar percent={clampPercent(percent)} />
          </div>
        ))}
      </div>
    </section>
  );
}
