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
