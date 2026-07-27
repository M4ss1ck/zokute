import { useRef } from "react";
import { useAudioFrame } from "../useAudioFrame";
import type { SectionConfig, Stats } from "../useStats";

// Only the starting height for a newly added spectrum; after that the window
// box is whatever the user dragged it to.
export const SPECTRUM_HEIGHT = 160;

const GAP = 2;
const FLOOR = 2;

interface Props {
  stats: Stats;
  section: SectionConfig;
}

export function barRects(bands: Float32Array, width: number, height: number) {
  const slot = width / bands.length;
  return Array.from(bands, (value, index) => ({ x: index * slot, width: Math.max(1, slot - GAP), height: Math.max(FLOOR, value * height) }));
}

export function resolveFill(context: CanvasRenderingContext2D, section: SectionConfig, width: number, height: number, fallback: string) {
  const from = section.color_a ?? fallback;
  if (section.color_mode !== "gradient") return from;
  const gradient = section.gradient_direction === "vertical"
    ? context.createLinearGradient(0, 0, 0, height)
    : context.createLinearGradient(0, 0, width, 0);
  gradient.addColorStop(0, from);
  gradient.addColorStop(1, section.color_b ?? from);
  return gradient;
}

export function SpectrumWidget({ stats, section }: Props) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const fallback = stats.config.graph_color ?? "#494137";
  useAudioFrame(canvasRef, (context, bands, alpha, width, height) => {
    context.globalAlpha = alpha;
    context.fillStyle = resolveFill(context, section, width, height, fallback);
    for (const bar of barRects(bands, width, height)) context.fillRect(bar.x, height - bar.height, bar.width, bar.height);
  });
  return <canvas className="vizCanvas vizCanvas--fill" ref={canvasRef} aria-hidden="true" />;
}
