import { IconNetwork } from "@tabler/icons-react";
import { Sparkline } from "../viz/Sparkline";
import { formatRate } from "../format";
import { aggregateNetworkRates } from "../network-rates";
import type { Stats, StatsHistory } from "../useStats";

interface Props {
  stats: Stats;
  history: StatsHistory;
}

export function NetworkWidget({ stats, history }: Props) {
  const showHeader = stats.profile.sections.find((section) => section.id === "network")?.show_header ?? true;
  const byteMode = (stats.config.byte_format ?? "binary") as "binary" | "decimal";
  const network = aggregateNetworkRates(stats.network);
  const rows = [
    { label: "Down", value: formatRate(network.down_bytes_per_second, byteMode), values: history.networkDown },
    { label: "Up", value: formatRate(network.up_bytes_per_second, byteMode), values: history.networkUp },
  ];
  return (
    <section className="panel">
      {showHeader ? (
        <header className="panelHeader">
          <span className="panelTitleGroup">
            <IconNetwork className="panelIcon" />
            <span className="panelTitle">Network</span>
          </span>
        </header>
      ) : null}
      <div className="networkStack">
        {rows.map((row) => (
          <div className="networkRow" key={row.label}>
            <div className="metric">
              <span className="metricLabel">{row.label}</span>
              <span className="metricValue">{row.value}</span>
            </div>
            <div className="networkSparklineRow">
              <Sparkline values={row.values} />
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
