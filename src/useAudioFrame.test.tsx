import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { useRef } from "react";
import { useAudioFrame } from "./useAudioFrame";

const listeners: Array<(event: { payload: { bands: number[] } }) => void> = [];

vi.mock("@tauri-apps/api/event", () => ({
  listen: (_name: string, handler: (event: { payload: { bands: number[] } }) => void) => {
    listeners.push(handler);
    return Promise.resolve(() => {});
  },
}));

afterEach(cleanup);

afterEach(() => {
  listeners.length = 0;
  vi.restoreAllMocks();
});

function harness(draw: (alpha: number) => void) {
  function Probe() {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    useAudioFrame(canvasRef, (_context, _bands, alpha) => draw(alpha));
    return <canvas ref={canvasRef} />;
  }
  return Probe;
}

function stubCanvas() {
  vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue({
    setTransform: vi.fn(),
    clearRect: vi.fn(),
  } as unknown as CanvasRenderingContext2D);
}

it("draws on frame arrival and stops the loop once the fade completes", async () => {
  stubCanvas();
  const pending: FrameRequestCallback[] = [];
  vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
    pending.push(callback);
    return pending.length;
  });
  vi.stubGlobal("cancelAnimationFrame", vi.fn());
  let now = 1000;
  vi.spyOn(performance, "now").mockImplementation(() => now);

  const alphas: number[] = [];
  const Probe = harness((alpha) => alphas.push(alpha));
  const { container } = render(<Probe />);
  const canvas = container.querySelector("canvas") as HTMLCanvasElement;
  await Promise.resolve();
  expect(canvas.style.visibility).toBe("hidden");

  listeners[0]({ payload: { bands: Array(96).fill(255) } });
  expect(pending).toHaveLength(1);

  pending.shift()!(0);
  expect(alphas.at(-1)).toBe(1);
  expect(pending).toHaveLength(1);

  now += 5000;
  pending.shift()!(0);
  expect(alphas.at(-1)).toBe(0);
  expect(canvas.style.visibility).toBe("hidden");
  expect(pending).toHaveLength(0);

  listeners[0]({ payload: { bands: Array(96).fill(255) } });
  expect(canvas.style.visibility).toBe("visible");
  expect(pending).toHaveLength(1);
});
