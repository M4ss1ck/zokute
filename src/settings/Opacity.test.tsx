import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { OpacityControl } from "./Opacity";

// This project does not set vitest `globals`, so Testing Library never
// registers its automatic cleanup and renders would otherwise accumulate.
afterEach(cleanup);

it("previews on input and commits on change", () => {
  const onPreview = vi.fn();
  const onCommit = vi.fn();
  const { getByLabelText } = render(
    <OpacityControl value={0.5} onPreview={onPreview} onCommit={onCommit} />,
  );
  const slider = getByLabelText("Opacity") as HTMLInputElement;
  fireEvent.input(slider, { target: { value: "0.75" } });
  expect(onPreview).toHaveBeenCalledWith(0.75);
  expect(onCommit).not.toHaveBeenCalled();
  fireEvent.change(slider, { target: { value: "0.75" } });
  expect(onCommit).toHaveBeenCalledWith(0.75);
});

it("shows the value as a percentage", () => {
  const { getByText } = render(
    <OpacityControl value={0.92} onPreview={vi.fn()} onCommit={vi.fn()} />,
  );
  getByText("92%");
});
