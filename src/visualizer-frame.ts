import type { SectionConfig } from "./useStats";

// The shared capture emits one normalized spectrum on a fixed logarithmic
// scale; per-instance parameters only choose which slice of it to draw.
export const SPECTRUM_MIN_HZ = 20;
export const SPECTRUM_MAX_HZ = 22000;

const LOG_FLOOR = Math.log2(SPECTRUM_MIN_HZ);
const LOG_SPAN = Math.log2(SPECTRUM_MAX_HZ) - LOG_FLOOR;
const BAR_FLOOR = 2;

export interface VisualizerParams {
  barCount: number;
  minHz: number;
  maxHz: number;
  gain: number;
  smoothing: number;
  decay: number;
  mirror: boolean;
  gap: number;
  roundedCaps: boolean;
  fps: number;
}

function clamp(value: number, low: number, high: number) {
  return Math.min(high, Math.max(low, value));
}

export function resolveParams(section: SectionConfig): VisualizerParams {
  const maxHz = clamp(section.viz_max_hz ?? 16000, 2000, SPECTRUM_MAX_HZ);
  // The two frequency windows overlap at 2000 Hz, so a user can land on an
  // empty range. Keep the maximum they asked for and drop the minimum below it.
  const minHz = clamp(Math.min(section.viz_min_hz ?? 40, maxHz / 2), SPECTRUM_MIN_HZ, 2000);
  return {
    barCount: Math.round(clamp(section.viz_bar_count ?? 48, 8, 128)),
    minHz,
    maxHz,
    gain: clamp(section.viz_gain ?? 1.0, 0.1, 5.0),
    smoothing: clamp(section.viz_smoothing ?? 0.65, 0.0, 0.95),
    decay: clamp(section.viz_decay ?? 0.82, 0.0, 0.99),
    mirror: section.viz_mirror ?? false,
    gap: Math.max(0, section.viz_gap ?? 4),
    roundedCaps: section.viz_rounded_caps ?? false,
    fps: (section.viz_fps ?? 30) >= 45 ? 60 : 30,
  };
}

/// Index of the capture bin holding `freq`, on the capture's logarithmic scale.
export function binForFreq(freq: number, bins: number) {
  const t = (Math.log2(freq) - LOG_FLOOR) / LOG_SPAN;
  return clamp(Math.round(t * (bins - 1)), 0, bins - 1);
}

/// Normalized 0..1 magnitude for the `index`th of `count` bands spread
/// logarithmically across the parameters' frequency window.
export function bandValue(spectrum: Float32Array, params: VisualizerParams, index: number, count: number) {
  const logMin = Math.log2(params.minHz);
  const logRange = Math.log2(params.maxHz) - logMin;
  const t = count > 1 ? index / (count - 1) : 0;
  const freq = Math.pow(2, logMin + t * logRange);
  const raw = spectrum[binForFreq(freq, spectrum.length)] ?? 0;
  return clamp(raw * params.gain, 0, 1);
}

export interface BarRect {
  x: number;
  y: number;
  width: number;
  height: number;
}

export function mapBands(
  spectrum: Float32Array,
  params: VisualizerParams,
  width: number,
  height: number,
): BarRect[] {
  const total = params.barCount;
  const barWidth = (width - (total - 1) * params.gap) / total;
  const bars: BarRect[] = [];
  for (let bar = 0; bar < total; bar++) {
    const value = bandValue(spectrum, params, bar, total);
    const drawn = Math.max(BAR_FLOOR, value * height);
    bars.push({
      x: bar * (barWidth + params.gap),
      y: params.mirror ? 0 : height - drawn,
      width: barWidth,
      height: drawn,
    });
  }
  return bars;
}
