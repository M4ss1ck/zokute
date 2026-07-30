import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { SettingsFooter } from "./SettingsFooter";

afterEach(cleanup);

function renderFooter(overrides: Partial<Parameters<typeof SettingsFooter>[0]> = {}) {
  const props = {
    arranging: false,
    dirty: false,
    onArrangeChange: vi.fn(),
    onSave: vi.fn(),
    onDiscard: vi.fn(),
    ...overrides,
  };
  return { props, ...render(<SettingsFooter {...props} />) };
}

it("disables Save and Discard while the draft is clean", () => {
  const { getByRole } = renderFooter();
  expect(getByRole("button", { name: "Save" })).toBeDisabled();
  expect(getByRole("button", { name: "Discard" })).toBeDisabled();
});

it("enables Save and Discard once the draft is dirty", () => {
  const { getByRole } = renderFooter({ dirty: true });
  expect(getByRole("button", { name: "Save" })).toBeEnabled();
  expect(getByRole("button", { name: "Discard" })).toBeEnabled();
});

it("reports Save and Discard clicks", () => {
  const { props, getByRole } = renderFooter({ dirty: true });
  fireEvent.click(getByRole("button", { name: "Save" }));
  fireEvent.click(getByRole("button", { name: "Discard" }));
  expect(props.onSave).toHaveBeenCalledTimes(1);
  expect(props.onDiscard).toHaveBeenCalledTimes(1);
});

it("reports the arrange toggle flipping on", () => {
  const { props, getByRole } = renderFooter();
  fireEvent.click(getByRole("switch", { name: "Arrange widgets" }));
  expect(props.onArrangeChange).toHaveBeenCalledWith(true);
});

it("shows the arrange toggle as on when arranging", () => {
  const { getByRole } = renderFooter({ arranging: true });
  expect(getByRole("switch", { name: "Arrange widgets" })).toBeChecked();
});
