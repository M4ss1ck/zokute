export interface NetworkReading {
  name: string;
  connected: boolean;
  down_bytes_per_second: number;
  up_bytes_per_second: number;
}

const ignoredPrefixes = ["lo", "docker", "br-", "veth", "vnet", "tun", "tap", "virbr"];

export function aggregateNetworkRates(readings: NetworkReading[]) {
  return readings
    .filter((reading) =>
      reading.connected && !ignoredPrefixes.some((prefix) => reading.name.startsWith(prefix))
    )
    .reduce(
      (total, reading) => ({
        down_bytes_per_second: total.down_bytes_per_second + reading.down_bytes_per_second,
        up_bytes_per_second: total.up_bytes_per_second + reading.up_bytes_per_second,
      }),
      { down_bytes_per_second: 0, up_bytes_per_second: 0 },
    );
}
