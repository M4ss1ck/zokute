import { expect, it } from "vitest";
import { render } from "@testing-library/react";
import { Sparkline } from "./Sparkline";

it("renders an empty sparkline for fewer than two samples", () => {
  const { container } = render(<Sparkline values={[42]} />);

  const svg = container.querySelector("svg");
  const polyline = container.querySelector("polyline");

  expect(svg).toBeInTheDocument();
  expect(polyline).toBeNull();
});

it("auto-scales finite samples when bounds are omitted", () => {
  const { container } = render(<Sparkline values={[5, 10]} width={100} height={20} />);

  expect(container.querySelector("polyline")?.getAttribute("points")).toBe("0.0,20.0 100.0,0.0");
});

it("keeps explicit bounds fixed", () => {
  const { container } = render(<Sparkline values={[5, 10]} min={0} max={100} width={100} height={20} />);

  expect(container.querySelector("polyline")?.getAttribute("points")).toBe("0.0,19.0 100.0,18.0");
});

it("centers equal samples without NaN", () => {
  const { container } = render(<Sparkline values={[7, 7]} width={100} height={20} />);

  expect(container.querySelector("polyline")?.getAttribute("points")).toBe("0.0,10.0 100.0,10.0");
});

it("drops non-finite samples when autoscaling and plotting", () => {
  const { container } = render(<Sparkline values={[5, Number.NaN, 10, Number.POSITIVE_INFINITY]} width={100} height={20} />);

  const points = container.querySelector("polyline")?.getAttribute("points") ?? "";

  expect(points).toBe("0.0,20.0 100.0,0.0");
  expect(points).not.toMatch(/NaN|Infinity/);
});

it("returns an empty sparkline when fewer than two finite samples remain", () => {
  const { container } = render(<Sparkline values={[Number.NaN, 42]} min={0} max={100} />);

  expect(container.querySelector("polyline")).toBeNull();
});

it("treats non-finite explicit bounds as omitted and normalizes reversed bounds", () => {
  const invalidBounds = render(<Sparkline values={[5, 10]} min={0} max={Number.POSITIVE_INFINITY} width={100} height={20} />);
  const reversedBounds = render(<Sparkline values={[0, 100]} min={100} max={0} width={100} height={20} />);

  expect(invalidBounds.container.querySelector("polyline")?.getAttribute("points")).toBe("0.0,20.0 100.0,0.0");
  expect(reversedBounds.container.querySelector("polyline")?.getAttribute("points")).toBe("0.0,20.0 100.0,0.0");
});

it("uses percentage width, numeric viewBox, and non-scaling stroke", () => {
  const { container } = render(<Sparkline values={[1, 2]} width={100} height={20} />);

  const svg = container.querySelector("svg");
  const polyline = container.querySelector("polyline");

  expect(svg?.getAttribute("width")).toBe("100%");
  expect(svg?.getAttribute("viewBox")).toBe("0 0 100 20");
  expect(svg?.getAttribute("preserveAspectRatio")).toBe("none");
  expect(polyline?.getAttribute("vector-effect")).toBe("non-scaling-stroke");
});
