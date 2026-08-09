import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import { AlignmentGuides } from "./AlignmentGuides";

afterEach(cleanup);

it("renders nothing when there is nothing to align to", () => {
  const { container } = render(<AlignmentGuides guides={[]} />);
  expect(container.firstChild).toBeNull();
});

it("places a vertical guide by its left offset", () => {
  const { container } = render(
    <AlignmentGuides guides={[{ orientation: "Vertical", position: 960 }]} />,
  );
  const line = container.querySelector(
    ".alignmentGuideVertical",
  ) as HTMLElement;
  expect(line).not.toBeNull();
  expect(line.style.left).toBe("960px");
});

it("places a horizontal guide by its top offset", () => {
  const { container } = render(
    <AlignmentGuides guides={[{ orientation: "Horizontal", position: 540 }]} />,
  );
  const line = container.querySelector(
    ".alignmentGuideHorizontal",
  ) as HTMLElement;
  expect(line).not.toBeNull();
  expect(line.style.top).toBe("540px");
});

it("draws both axes at once", () => {
  const { container } = render(
    <AlignmentGuides
      guides={[
        { orientation: "Vertical", position: 960 },
        { orientation: "Horizontal", position: 540 },
      ]}
    />,
  );
  expect(container.querySelectorAll(".alignmentGuideVertical")).toHaveLength(1);
  expect(container.querySelectorAll(".alignmentGuideHorizontal")).toHaveLength(
    1,
  );
});
