import { cleanup } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { SectionConfig } from "../useStats";
import { resolveFill } from "./Spectrum";

afterEach(cleanup);

function section(extra: Partial<SectionConfig> = {}): SectionConfig {
  return { id: "spectrum", instance: "spectrum", enabled: true, monitor: 0, x: 0, y: 0, width: 1920, ...extra };
}

it("uses the fallback color when the section sets none", () => {
  const context = { createLinearGradient: vi.fn() } as unknown as CanvasRenderingContext2D;
  expect(resolveFill(context, section(), 100, 50, "#494137")).toBe("#494137");
  expect(context.createLinearGradient).not.toHaveBeenCalled();
});

it("builds a two-stop gradient only in gradient mode", () => {
  const gradient = { addColorStop: vi.fn() };
  const context = { createLinearGradient: vi.fn(() => gradient) } as unknown as CanvasRenderingContext2D;
  const result = resolveFill(context, section({ color_mode: "gradient", color_a: "#c07100", color_b: "#2563eb" }), 500, 100, "#494137");
  expect(result).toBe(gradient);
  expect(context.createLinearGradient).toHaveBeenCalledWith(0, 0, 500, 0);
  expect(gradient.addColorStop).toHaveBeenCalledWith(0, "#c07100");
  expect(gradient.addColorStop).toHaveBeenCalledWith(1, "#2563eb");
});

it("builds a top-to-bottom gradient in vertical mode", () => {
  const gradient = { addColorStop: vi.fn() };
  const context = { createLinearGradient: vi.fn(() => gradient) } as unknown as CanvasRenderingContext2D;
  resolveFill(context, section({ color_mode: "gradient", gradient_direction: "vertical" }), 500, 100, "#494137");
  expect(context.createLinearGradient).toHaveBeenCalledWith(0, 0, 0, 100);
});

it("falls back to a solid fill when gradient mode has no second color", () => {
  const gradient = { addColorStop: vi.fn() };
  const context = { createLinearGradient: vi.fn(() => gradient) } as unknown as CanvasRenderingContext2D;
  resolveFill(context, section({ color_mode: "gradient", color_a: "#c07100" }), 500, 100, "#494137");
  expect(gradient.addColorStop).toHaveBeenCalledWith(1, "#c07100");
});

vi.mock("../useAudioFrame", () => ({ useAudioFrame: () => {} }));

it("lets the canvas fill the window box in both axes", async () => {
  const { render } = await import("@testing-library/react");
  const { SpectrumWidget } = await import("./Spectrum");
  const stats = { config: { graph_color: "#494137" }, profile: { sections: [], system_fields: [], show_cpu_cores: true, disks: [] } } as unknown as import("../useStats").Stats;
  const { container } = render(<SpectrumWidget stats={stats} section={section()} />);
  const canvas = container.querySelector("canvas");
  expect(canvas?.className).toContain("vizCanvas--fill");
  expect(canvas?.getAttribute("style")).toBeNull();
});
