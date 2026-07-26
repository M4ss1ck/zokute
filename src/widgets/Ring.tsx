import { useRef } from "react";
import { useAudioFrame } from "../useAudioFrame";
import type { SectionConfig, Stats } from "../useStats";

// Only the starting size for a newly added ring; after that the window box is
// whatever the user dragged it to.
export const RING_SIZE = 420;

const INNER = 0.45;
const MARGIN = 4;
const FLOOR = 2;
const STROKE = 3;

interface Props {
  stats: Stats;
  section: SectionConfig;
}

export function ringSpokes(bands: Float32Array, size: number) {
  const center = size / 2;
  const inner = center * INNER;
  const span = center - inner - MARGIN;
  return Array.from(bands, (value, index) => {
    const angle = (index / bands.length) * Math.PI * 2 - Math.PI / 2;
    const outer = inner + Math.max(FLOOR, value * span);
    return { x1: center + Math.cos(angle) * inner, y1: center + Math.sin(angle) * inner, x2: center + Math.cos(angle) * outer, y2: center + Math.sin(angle) * outer };
  });
}

export function resolveFill(context: CanvasRenderingContext2D, section: SectionConfig, size: number, fallback: string) {
  const from = section.color_a ?? fallback;
  if (section.color_mode !== "gradient") return from;
  const gradient = context.createLinearGradient(0, 0, size, size);
  gradient.addColorStop(0, from);
  gradient.addColorStop(1, section.color_b ?? from);
  return gradient;
}

export function RingWidget({ stats, section }: Props) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const fallback = stats.config.graph_color ?? "#494137";
  useAudioFrame(canvasRef, (context, bands, alpha, width) => {
    context.globalAlpha = alpha;
    context.strokeStyle = resolveFill(context, section, width, fallback);
    context.lineWidth = STROKE;
    context.lineCap = "round";
    context.beginPath();
    for (const spoke of ringSpokes(bands, width)) { context.moveTo(spoke.x1, spoke.y1); context.lineTo(spoke.x2, spoke.y2); }
    context.stroke();
  });
  return <canvas className="vizCanvas vizCanvas--square" ref={canvasRef} aria-hidden="true" />;
}
