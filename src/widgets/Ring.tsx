import { useRef } from "react";
import { useAudioFrame } from "../useAudioFrame";
import { resolveParams, type VisualizerParams } from "../visualizer-frame";
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

function ringSpokesWithParams(bands: Float32Array, size: number, params: VisualizerParams) {
  const center = size / 2;
  const inner = center * INNER;
  const span = center - inner - MARGIN;
  const count = params.barCount;
  const bins = bands.length;
  const logMin = Math.log2(params.minHz);
  const logMax = Math.log2(params.maxHz);
  const logRange = logMax - logMin;
  return Array.from({ length: count }, (_, index) => {
    const t = index / count;
    const freq = Math.pow(2, logMin + t * logRange);
    const binIndex = Math.round((freq / 22000) * bins);
    const raw = bands[Math.min(binIndex, bins - 1)] ?? 0;
    const value = Math.pow(raw, params.gain);
    const angle = (index / count) * Math.PI * 2 - Math.PI / 2;
    const outer = inner + Math.max(FLOOR, value * span);
    return { x1: center + Math.cos(angle) * inner, y1: center + Math.sin(angle) * inner, x2: center + Math.cos(angle) * outer, y2: center + Math.sin(angle) * outer };
  });
}

export function resolveFill(context: CanvasRenderingContext2D, section: SectionConfig, size: number, fallback: string) {
  const from = section.color_a ?? fallback;
  if (section.color_mode !== "gradient") return from;
  const gradient = section.gradient_direction === "vertical"
    ? context.createLinearGradient(0, 0, 0, size)
    : context.createLinearGradient(0, 0, size, 0);
  gradient.addColorStop(0, from);
  gradient.addColorStop(1, section.color_b ?? from);
  return gradient;
}

export function RingWidget({ stats, section }: Props) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const fallback = stats.config.graph_color ?? "#494137";
  const params = resolveParams(section);
  useAudioFrame(canvasRef, section, (context, bands, alpha, width) => {
    context.globalAlpha = alpha;
    context.strokeStyle = resolveFill(context, section, width, fallback);
    context.lineWidth = STROKE;
    context.lineCap = params.roundedCaps ? "round" : "butt";
    context.beginPath();
    for (const spoke of ringSpokesWithParams(bands, width, params)) { context.moveTo(spoke.x1, spoke.y1); context.lineTo(spoke.x2, spoke.y2); }
    context.stroke();
  });
  return <canvas className="vizCanvas vizCanvas--square" ref={canvasRef} aria-hidden="true" />;
}
