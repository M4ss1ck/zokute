import { cleanup } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { SectionConfig } from "../useStats";
import { resolveFill } from "./Ring";

afterEach(cleanup);

function section(extra: Partial<SectionConfig> = {}): SectionConfig {
  return { id: "ring", instance: "ring", enabled: true, monitor: 0, x: 0, y: 0, width: 420, ...extra };
}

it("builds a horizontal gradient by default", () => {
  const gradient = { addColorStop: vi.fn() };
  const context = { createLinearGradient: vi.fn(() => gradient) } as unknown as CanvasRenderingContext2D;
  const result = resolveFill(context, section({ color_mode: "gradient", color_a: "#c07100", color_b: "#2563eb" }), 400, "#494137");
  expect(result).toBe(gradient);
  expect(context.createLinearGradient).toHaveBeenCalledWith(0, 0, 400, 0);
});

it("builds a vertical gradient when selected", () => {
  const gradient = { addColorStop: vi.fn() };
  const context = { createLinearGradient: vi.fn(() => gradient) } as unknown as CanvasRenderingContext2D;
  resolveFill(context, section({ color_mode: "gradient", gradient_direction: "vertical" }), 400, "#494137");
  expect(context.createLinearGradient).toHaveBeenCalledWith(0, 0, 0, 400);
});

it("uses the fallback color when the section sets none", () => {
  const context = { createLinearGradient: vi.fn() } as unknown as CanvasRenderingContext2D;
  expect(resolveFill(context, section(), 400, "#494137")).toBe("#494137");
});

vi.mock("../useAudioFrame", () => ({ useAudioFrame: () => {} }));

it("keeps the canvas square inside the window box", async () => {
  const { render } = await import("@testing-library/react");
  const { RingWidget } = await import("./Ring");
  const stats = { config: { graph_color: "#494137" }, profile: { sections: [], system_fields: [], show_cpu_cores: true, disks: [] } } as unknown as import("../useStats").Stats;
  const { container } = render(<RingWidget stats={stats} section={section()} />);
  const canvas = container.querySelector("canvas");
  expect(canvas?.className).toContain("vizCanvas--square");
  expect(canvas?.getAttribute("style")).toBeNull();
});
