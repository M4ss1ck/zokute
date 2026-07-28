import type { SectionConfig } from "./useStats";

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

export function resolveParams(section: SectionConfig): VisualizerParams {
  return {
    barCount: section.viz_bar_count ?? 64,
    minHz: section.viz_min_hz ?? 20,
    maxHz: section.viz_max_hz ?? 22000,
    gain: section.viz_gain ?? 1.0,
    smoothing: section.viz_smoothing ?? 0.8,
    decay: section.viz_decay ?? 0.3,
    mirror: section.viz_mirror ?? true,
    gap: section.viz_gap ?? 1,
    roundedCaps: section.viz_rounded_caps ?? false,
    fps: section.viz_fps ?? 30,
  };
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
  const bins = spectrum.length;
  const totalBars = params.mirror
    ? Math.ceil(params.barCount / 2)
    : params.barCount;
  const barWidth = (width - (totalBars - 1) * params.gap) / totalBars;
  const logMin = Math.log2(params.minHz);
  const logMax = Math.log2(params.maxHz);
  const logRange = logMax - logMin;

  const bars: BarRect[] = [];
  for (let i = 0; i < totalBars; i++) {
    const t = i / totalBars;
    const freq = Math.pow(2, logMin + t * logRange);
    const binIndex = Math.round((freq / 22000) * bins);
    const raw = spectrum[Math.min(binIndex, bins - 1)] ?? 0;
    const value = Math.pow(raw, params.gain) * height;
    const clamped = Math.max(0, Math.min(height, value));
    bars.push({
      x: i * (barWidth + params.gap),
      y: height - clamped,
      width: barWidth,
      height: clamped,
    });
    if (params.mirror && i > 0) {
      bars.unshift({
        x: width - (i * (barWidth + params.gap) + barWidth),
        y: height - clamped,
        width: barWidth,
        height: clamped,
      });
    }
  }
  return bars;
}
