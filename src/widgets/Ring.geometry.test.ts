import { expect, it } from "vitest";
import { ringSpokes } from "./Ring";
import { resolveParams } from "../visualizer-frame";
import type { SectionConfig } from "../useStats";

function params(extra: Partial<SectionConfig> = {}) {
  return resolveParams({ id: "ring", instance: "ring", enabled: true, viz_mirror: false, ...extra } as SectionConfig);
}

const SIZE = 400;
const CENTRE = SIZE / 2;

it("places the first spoke at twelve o'clock", () => {
  const [spoke] = ringSpokes(new Float32Array([1, 0, 0, 0]), SIZE, params({ viz_bar_count: 8 }));
  expect(spoke.x1).toBeCloseTo(CENTRE, 5);
  expect(spoke.y1).toBeLessThan(CENTRE);
  expect(spoke.x2).toBeCloseTo(CENTRE, 5);
  expect(spoke.y2).toBeLessThan(spoke.y1);
});

it("spaces spokes evenly around the full turn", () => {
  const spokes = ringSpokes(new Float32Array(64), SIZE, params({ viz_bar_count: 8 }));
  expect(spokes[2].x1).toBeGreaterThan(CENTRE);
  expect(spokes[2].y1).toBeCloseTo(CENTRE, 5);
  expect(spokes[4].y1).toBeGreaterThan(CENTRE);
  expect(spokes[4].x1).toBeCloseTo(CENTRE, 5);
});

it("keeps a silent ring visible as a thin rim", () => {
  const [spoke] = ringSpokes(new Float32Array(64), SIZE, params({ viz_bar_count: 8 }));
  expect(Math.hypot(spoke.x2 - spoke.x1, spoke.y2 - spoke.y1)).toBeCloseTo(2, 5);
});

it("draws exactly bar_count spokes regardless of the capture resolution", () => {
  expect(ringSpokes(new Float32Array(512), SIZE, params({ viz_bar_count: 32 }))).toHaveLength(32);
});

it("mirrors spokes symmetrically about the vertical axis", () => {
  const spectrum = new Float32Array(128);
  spectrum.fill(0.4);
  spectrum[90] = 1;
  const spokes = ringSpokes(spectrum, SIZE, params({ viz_bar_count: 8, viz_mirror: true }));
  const reach = spokes.map((spoke) => Math.hypot(spoke.x2 - spoke.x1, spoke.y2 - spoke.y1));
  expect(reach[1]).toBeCloseTo(reach[7], 5);
  expect(reach[2]).toBeCloseTo(reach[6], 5);
});

it("reads the logarithmic bin a spoke covers rather than a linear one", () => {
  const spectrum = new Float32Array(101);
  spectrum[50] = 1;
  const loud = ringSpokes(spectrum, SIZE, params({ viz_bar_count: 101, viz_min_hz: 20, viz_max_hz: 22000 }));
  const reach = loud.map((spoke) => Math.hypot(spoke.x2 - spoke.x1, spoke.y2 - spoke.y1));
  expect(reach[50]).toBeGreaterThan(reach[0]);
  expect(reach[50]).toBeGreaterThan(reach[100]);
  expect(reach[0]).toBeCloseTo(2, 5);
});

it("multiplies the normalized input by gain rather than exponentiating it", () => {
  const spectrum = new Float32Array(101);
  spectrum.fill(0.25);
  const single = ringSpokes(spectrum, SIZE, params({ viz_bar_count: 8, viz_gain: 1 }));
  const doubled = ringSpokes(spectrum, SIZE, params({ viz_bar_count: 8, viz_gain: 2 }));
  const reach = (spokes: typeof single) => Math.hypot(spokes[0].x2 - spokes[0].x1, spokes[0].y2 - spokes[0].y1);
  expect(reach(doubled)).toBeCloseTo(reach(single) * 2, 5);
});
