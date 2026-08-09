import { expect, it } from "vitest";
import { resolveParams, mapBands, binForFreq, SPECTRUM_MIN_HZ, SPECTRUM_MAX_HZ } from "./visualizer-frame";
import type { SectionConfig } from "./useStats";

function section(extra: Partial<SectionConfig> = {}): SectionConfig {
  return { id: "spectrum", instance: "spectrum", enabled: true, ...extra } as SectionConfig;
}

// AUDIO-003 initial configuration.
it("falls back to the specified initial configuration", () => {
  expect(resolveParams(section())).toMatchObject({
    barCount: 48, gain: 1.0, smoothing: 0.65, decay: 0.82,
    minHz: 40, maxHz: 16000, mirror: false, roundedCaps: false, gap: 4,
  });
});

// AUDIO-003 recommended bounds.
it("clamps bar count into 8-128", () => {
  expect(resolveParams(section({ viz_bar_count: 2 })).barCount).toBe(8);
  expect(resolveParams(section({ viz_bar_count: 999 })).barCount).toBe(128);
});

it("clamps gain into 0.1-5.0", () => {
  expect(resolveParams(section({ viz_gain: 0 })).gain).toBe(0.1);
  expect(resolveParams(section({ viz_gain: 50 })).gain).toBe(5.0);
});

it("clamps smoothing into 0.0-0.95", () => {
  expect(resolveParams(section({ viz_smoothing: -1 })).smoothing).toBe(0);
  expect(resolveParams(section({ viz_smoothing: 1 })).smoothing).toBe(0.95);
});

it("clamps decay into 0.0-0.99", () => {
  expect(resolveParams(section({ viz_decay: -1 })).decay).toBe(0);
  expect(resolveParams(section({ viz_decay: 5 })).decay).toBe(0.99);
});

it("clamps the frequency range into its two windows", () => {
  expect(resolveParams(section({ viz_min_hz: 1 })).minHz).toBe(20);
  expect(resolveParams(section({ viz_min_hz: 9000 })).minHz).toBe(2000);
  expect(resolveParams(section({ viz_max_hz: 100 })).maxHz).toBe(2000);
  expect(resolveParams(section({ viz_max_hz: 90000 })).maxHz).toBe(22000);
});

it("keeps the minimum frequency below the maximum", () => {
  const params = resolveParams(section({ viz_min_hz: 2000, viz_max_hz: 2000 }));
  expect(params.minHz).toBeLessThan(params.maxHz);
});

// AUDIO-002 frame caps.
it("accepts only the two supported frame caps", () => {
  expect(resolveParams(section({ viz_fps: 60 })).fps).toBe(60);
  expect(resolveParams(section({ viz_fps: 144 })).fps).toBe(60);
  expect(resolveParams(section({ viz_fps: 5 })).fps).toBe(30);
});

// AUDIO-004: frequency grouping is logarithmic over the captured range.
it("maps a frequency to its bin on the logarithmic capture scale", () => {
  expect(binForFreq(SPECTRUM_MIN_HZ, 101)).toBe(0);
  expect(binForFreq(SPECTRUM_MAX_HZ, 101)).toBe(100);
  const middle = Math.sqrt(SPECTRUM_MIN_HZ * SPECTRUM_MAX_HZ);
  expect(binForFreq(middle, 101)).toBe(50);
});

it("reads the logarithmic bin a bar covers rather than a linear one", () => {
  const spectrum = new Float32Array(101);
  spectrum[50] = 1;
  const params = resolveParams(section({ viz_bar_count: 101, viz_mirror: false, viz_min_hz: 20, viz_max_hz: 22000, viz_gap: 0 }));
  const bars = mapBands(spectrum, params, 101, 100);
  expect(bars[50].height).toBe(100);
  expect(bars[0].height).toBeLessThan(5);
  expect(bars[100].height).toBeLessThan(5);
});

// AUDIO-004: bar_count is the total number of visible bars.
it("draws exactly bar_count bars when mirroring is off", () => {
  const params = resolveParams(section({ viz_bar_count: 64, viz_mirror: false }));
  expect(mapBands(new Float32Array(256), params, 640, 100)).toHaveLength(64);
});

it("draws exactly bar_count bars when mirroring is on", () => {
  const params = resolveParams(section({ viz_bar_count: 64, viz_mirror: true }));
  expect(mapBands(new Float32Array(256), params, 640, 100)).toHaveLength(64);
});

it("anchors mirrored bars at the top", () => {
  const params = resolveParams(section({ viz_bar_count: 8, viz_mirror: true, viz_gap: 0 }));
  const bars = mapBands(new Float32Array(128), params, 800, 100);
  expect(bars.every((bar) => bar.y === 0)).toBe(true);
});

it("keeps non-mirrored bars anchored at the bottom", () => {
  const params = resolveParams(section({ viz_bar_count: 8, viz_mirror: false, viz_gap: 0 }));
  const bars = mapBands(new Float32Array(128), params, 800, 100);
  expect(bars.every((bar) => bar.y + bar.height === 100)).toBe(true);
});

it("spans the full width with bars and gaps", () => {
  const params = resolveParams(section({ viz_bar_count: 10, viz_mirror: false, viz_gap: 2 }));
  const bars = mapBands(new Float32Array(256), params, 200, 100);
  expect(bars[0].x).toBe(0);
  const last = bars[bars.length - 1];
  expect(last.x + last.width).toBeCloseTo(200, 5);
});

// AUDIO-004: gain multiplies normalized input before clamping.
it("multiplies the normalized input by gain rather than exponentiating it", () => {
  const spectrum = new Float32Array(101);
  spectrum.fill(0.25);
  const params = resolveParams(section({ viz_bar_count: 8, viz_mirror: false, viz_gain: 2 }));
  const bars = mapBands(spectrum, params, 800, 100);
  expect(bars[0].height).toBeCloseTo(50, 5);
});

it("clamps a boosted band to the available height", () => {
  const spectrum = new Float32Array(101);
  spectrum.fill(1);
  const params = resolveParams(section({ viz_bar_count: 8, viz_mirror: false, viz_gain: 5 }));
  for (const bar of mapBands(spectrum, params, 800, 100)) {
    expect(bar.height).toBe(100);
    expect(bar.y).toBe(0);
  }
});

it("keeps silent bars visible as a hairline rather than nothing", () => {
  const params = resolveParams(section({ viz_bar_count: 8, viz_mirror: false }));
  for (const bar of mapBands(new Float32Array(101), params, 800, 100)) {
    expect(bar.height).toBe(2);
  }
});
