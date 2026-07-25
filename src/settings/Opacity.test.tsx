import { cleanup, fireEvent, render } from "@testing-library/react";
import { useState } from "react";
import { afterEach, expect, it, vi } from "vitest";
import { OpacityControl } from "./Opacity";

// This project does not set vitest `globals`, so Testing Library never
// registers its automatic cleanup and renders would otherwise accumulate.
afterEach(cleanup);

it("previews on input and commits on change", () => {
  const onPreview = vi.fn();
  const onCommit = vi.fn();
  function Harness() {
    const [value, setValue] = useState(0.5);
    return (
      <OpacityControl
        label="Opacity"
        value={value}
        onPreview={(next) => {
          setValue(next);
          onPreview(next);
        }}
        onCommit={onCommit}
      />
    );
  }
  const { getByRole } = render(
    <Harness />,
  );
  const slider = getByRole("slider", { name: "Opacity" }) as HTMLInputElement;
  const thumb = slider.closest(".settingsSliderThumb") as HTMLElement;
  fireEvent.mouseDown(thumb, { button: 0 });
  fireEvent.change(slider, { target: { value: "0.75" } });
  expect(onPreview).toHaveBeenCalledWith(0.75);
  expect(onCommit).not.toHaveBeenCalled();
  fireEvent.mouseUp(window);
  expect(onCommit).toHaveBeenCalledWith(0.75);
});

it("shows the value as a percentage", () => {
  const { getByText } = render(
    <OpacityControl label="Opacity" value={0.92} onPreview={vi.fn()} onCommit={vi.fn()} />,
  );
  getByText("92%");
});
