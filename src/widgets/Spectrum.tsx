import { useRef } from "react";
import { useAudioFrame } from "../useAudioFrame";
import { mapBands, resolveParams } from "../visualizer-frame";
import type { SectionConfig, Stats } from "../useStats";

// Only the starting height for a newly added spectrum; after that the window
// box is whatever the user dragged it to.
export const SPECTRUM_HEIGHT = 160;

const FLOOR = 2;

interface Props {
  stats: Stats;
  section: SectionConfig;
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
  const params = resolveParams(section);
  useAudioFrame(canvasRef, section, (context, bands, alpha, width, height) => {
    context.globalAlpha = alpha;
    context.fillStyle = resolveFill(context, section, width, height, fallback);
    const bars = mapBands(bands, params, width, height);
    for (const bar of bars) {
      const barHeight = Math.max(FLOOR, bar.height);
      if (params.roundedCaps) {
        const radius = bar.width / 2;
        context.beginPath();
        context.moveTo(bar.x, height);
        context.lineTo(bar.x, height - barHeight + radius);
        context.arcTo(bar.x, height - barHeight, bar.x + radius, height - barHeight, radius);
        context.arcTo(bar.x + bar.width, height - barHeight, bar.x + bar.width, height - barHeight + radius, radius);
        context.lineTo(bar.x + bar.width, height);
        context.closePath();
        context.fill();
      } else {
        context.fillRect(bar.x, height - barHeight, bar.width, barHeight);
      }
    }
  });
  return <canvas className="vizCanvas vizCanvas--fill" ref={canvasRef} aria-hidden="true" />;
}
