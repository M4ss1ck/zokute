import { IconThermometer } from "@tabler/icons-react";
import type { Stats } from "../useStats";

interface Props {
  stats: Stats;
}

function formatTemp(celsius: number) {
  return `${celsius.toFixed(1)}°C`;
}

export function TemperaturesWidget({ stats }: Props) {
  const cpuTemperature = stats.cpu_temperature;
  const gpuTemperatures = stats.gpu_temperatures;
  if (!cpuTemperature && gpuTemperatures.length === 0) return null;
  return (
    <section className="panel">
      <header className="panelHeader">
        <span className="panelTitleGroup">
          <IconThermometer className="panelIcon" />
          <span className="panelTitle">Temperatures</span>
        </span>
      </header>
      {cpuTemperature ? (
        <div className="metric">
          <span className="metricLabel">CPU</span>
          <span className="metricValue">{formatTemp(cpuTemperature.celsius)}</span>
        </div>
      ) : null}
      {gpuTemperatures.length > 0 ? (
        <div className="tempList">
          {gpuTemperatures.map((temperature, index) => (
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
