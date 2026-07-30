import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { SettingsClosePrompt } from "./SettingsClosePrompt";

afterEach(cleanup);

it("offers all three ways out", () => {
  const { getByRole } = render(
    <SettingsClosePrompt onSave={vi.fn()} onDiscard={vi.fn()} onKeepEditing={vi.fn()} />,
  );
  expect(getByRole("button", { name: "Save and close" })).not.toBeNull();
  expect(getByRole("button", { name: "Discard and close" })).not.toBeNull();
  expect(getByRole("button", { name: "Keep editing" })).not.toBeNull();
});

it("reports which way out was chosen", () => {
  const onSave = vi.fn();
  const onDiscard = vi.fn();
  const onKeepEditing = vi.fn();
  const { getByRole } = render(
    <SettingsClosePrompt onSave={onSave} onDiscard={onDiscard} onKeepEditing={onKeepEditing} />,
  );
  fireEvent.click(getByRole("button", { name: "Discard and close" }));
  expect(onDiscard).toHaveBeenCalledTimes(1);
  expect(onSave).not.toHaveBeenCalled();
  expect(onKeepEditing).not.toHaveBeenCalled();
});

it("treats Escape as keep editing so a stray key cannot lose the draft", () => {
  const onKeepEditing = vi.fn();
  const { getByRole } = render(
    <SettingsClosePrompt onSave={vi.fn()} onDiscard={vi.fn()} onKeepEditing={onKeepEditing} />,
  );
  fireEvent.keyDown(getByRole("dialog"), { key: "Escape" });
  expect(onKeepEditing).toHaveBeenCalledTimes(1);
});
