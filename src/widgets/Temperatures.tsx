import type { Stats } from "../useStats";

interface Props {
  stats: Stats;
}

function formatTemp(celsius: number) {
  return `${celsius.toFixed(1)}°C`;
}

export function TemperaturesWidget({ stats }: Props) {
  return (
    <section className="panel">
      <header className="panelHeader">
        <span className="panelTitle">Temperatures</span>
      </header>
      <div className="metric">
        <span className="metricLabel">CPU</span>
        <span className="metricValue">
          {stats.cpu_temperature ? `${stats.cpu_temperature.label} ${formatTemp(stats.cpu_temperature.celsius)}` : "CPU unavailable"}
        </span>
      </div>
      {stats.gpu_temperatures.length > 0 ? (
        <div className="tempList">
          {stats.gpu_temperatures.map((temperature, index) => (
            <div className="metric" key={`${temperature.label}-${index}`}>
              <span className="metricLabel">{temperature.label}</span>
              <span className="metricValue">{formatTemp(temperature.celsius)}</span>
            </div>
          ))}
        </div>
      ) : null}
    </section>
  );
}
