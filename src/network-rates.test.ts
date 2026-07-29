import { expect, it } from "vitest";
import { aggregateNetworkRates } from "./network-rates";

it("aggregates connected physical interfaces and ignores virtual traffic", () => {
  const rates = aggregateNetworkRates([
    { name: "enp6s0", connected: true, down_bytes_per_second: 1200, up_bytes_per_second: 200 },
    { name: "wlp4s0", connected: true, down_bytes_per_second: 300, up_bytes_per_second: 50 },
    { name: "docker0", connected: true, down_bytes_per_second: 9000, up_bytes_per_second: 8000 },
    { name: "enp7s0", connected: false, down_bytes_per_second: 7000, up_bytes_per_second: 6000 },
  ]);
  expect(rates).toEqual({ down_bytes_per_second: 1500, up_bytes_per_second: 250 });
});

it("returns finite zero rates when no physical interface is connected", () => {
  expect(aggregateNetworkRates([])).toEqual({
    down_bytes_per_second: 0,
    up_bytes_per_second: 0,
  });
});
