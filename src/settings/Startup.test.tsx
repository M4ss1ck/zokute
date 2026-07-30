import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { StartupToggle } from "./Startup";

afterEach(cleanup);

it("reflects the enabled state", () => {
  const { getByLabelText } = render(<StartupToggle enabled={true} onChange={() => {}} />);
  expect((getByLabelText("Start with the session") as HTMLInputElement).checked).toBe(true);
});

it("reports flips through onChange", () => {
  const onChange = vi.fn();
  const { getByLabelText } = render(<StartupToggle enabled={true} onChange={onChange} />);
  const toggle = getByLabelText("Start with the session") as HTMLInputElement;
  fireEvent.click(toggle);
  expect(onChange).toHaveBeenCalledWith(false);
});
