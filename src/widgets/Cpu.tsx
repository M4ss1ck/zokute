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
  const cpuTemperature = stats.cpu_temperature;
  const showHeader = stats.profile.sections.find((section) => section.id === "cpu")?.show_header ?? true;
  return (
    <section className="panel">
      {showHeader ? (
        <header className="panelHeader">
          <span className="panelTitleGroup">
            <IconCpu className="panelIcon" />
            <span className="panelTitle">CPU</span>
          </span>
          <span className="panelValue">{aggregate.toFixed(1)}%</span>
        </header>
      ) : null}
      {cpuTemperature ? (
        <div className="metric">
          <span className="metricLabel">{cpuTemperature.label}</span>
          <span className="metricValue">{`${cpuTemperature.celsius.toFixed(1)}°C`}</span>
        </div>
      ) : null}
      <Sparkline values={history.cpuAggregate} />
      {stats.profile.show_cpu_cores ? (
        <div className="coreGrid">
          {stats.cpu.core_percents.map((percent, index) => (
            <div className="coreCell" key={index}>
              <span className="coreLabel">C{index + 1}</span>
              <Bar percent={clampPercent(percent)} />
            </div>
          ))}
        </div>
      ) : null}
    </section>
  );
}
