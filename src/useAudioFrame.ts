import { listen } from "@tauri-apps/api/event";
import { useEffect, useRef, type RefObject } from "react";

const BANDS = 96;
const SILENCE_MS = 250;
const FADE_MS = 600;
const LERP = 0.35;

interface AudioFrame {
  bands: number[];
}

export type Draw = (
  context: CanvasRenderingContext2D,
  bands: Float32Array,
  alpha: number,
  width: number,
  height: number,
) => void;

// The one animation loop in the app. It exists only while audio is arriving:
// the last frame plus one fade, then it cancels itself and idle CPU is zero.
export function useAudioFrame(canvasRef: RefObject<HTMLCanvasElement | null>, draw: Draw) {
  const drawRef = useRef(draw);
  drawRef.current = draw;
  useEffect(() => {
    const target = new Float32Array(BANDS);
    const shown = new Float32Array(BANDS);
    let latest = 0;
    let handle = 0;
    let active = true;
    let unlisten = () => {};
    if (canvasRef.current) canvasRef.current.style.visibility = "hidden";

    const tick = () => {
      handle = 0;
      const canvas = canvasRef.current;
      const context = canvas?.getContext("2d");
      if (!canvas || !context) return;
      const quiet = performance.now() - latest;
      const alpha = quiet <= SILENCE_MS ? 1 : Math.max(0, 1 - (quiet - SILENCE_MS) / FADE_MS);
      for (let index = 0; index < BANDS; index += 1) {
        shown[index] += (target[index] - shown[index]) * LERP;
      }
      const ratio = globalThis.devicePixelRatio || 1;
      const width = canvas.clientWidth;
      const height = canvas.clientHeight;
      const backingWidth = Math.round(width * ratio);
      const backingHeight = Math.round(height * ratio);
      if (canvas.width !== backingWidth || canvas.height !== backingHeight) {
        canvas.width = backingWidth;
        canvas.height = backingHeight;
      }
      context.setTransform(ratio, 0, 0, ratio, 0, 0);
      context.clearRect(0, 0, width, height);
      drawRef.current(context, shown, alpha, width, height);
      if (alpha <= 0) {
        canvas.style.visibility = "hidden";
        shown.fill(0);
        return;
      }
      handle = requestAnimationFrame(tick);
    };

    void listen<AudioFrame>("audio", ({ payload }) => {
      if (!active) return;
      const canvas = canvasRef.current;
      if (canvas) canvas.style.visibility = "visible";
      for (let index = 0; index < BANDS; index += 1) {
        target[index] = (payload.bands[index] ?? 0) / 255;
      }
      latest = performance.now();
      if (handle === 0) handle = requestAnimationFrame(tick);
    }).then((cleanup) => {
      if (!active) {
        void cleanup();
        return;
      }
      unlisten = () => {
        void cleanup();
      };
    });

    return () => {
      active = false;
      if (handle !== 0) cancelAnimationFrame(handle);
      unlisten();
    };
  }, [canvasRef]);
}
