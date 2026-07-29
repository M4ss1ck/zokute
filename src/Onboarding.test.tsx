import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { Onboarding } from "./Onboarding";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn(() => Promise.resolve("")) }));

beforeEach(() => {
  document.documentElement.removeAttribute("data-theme");
});

afterEach(cleanup);

it("previews the default theme as soon as onboarding opens", () => {
  render(<Onboarding />);
  expect(document.documentElement.getAttribute("data-theme")).toBe("light");
});

it("applies the chosen theme immediately so the choice is visible", () => {
  const { getByText } = render(<Onboarding />);
  fireEvent.click(getByText("Atelier Dark"));
  expect(document.documentElement.getAttribute("data-theme")).toBe("dark");
});

it("switches back when another theme is chosen", () => {
  const { getByText } = render(<Onboarding />);
  fireEvent.click(getByText("Atelier Dark"));
  fireEvent.click(getByText("Atelier Light"));
  expect(document.documentElement.getAttribute("data-theme")).toBe("light");
});

it("marks the selected theme so the choice is not invisible", () => {
  const { getByText } = render(<Onboarding />);
  const dark = getByText("Atelier Dark");
  expect(dark.className).not.toContain("onboardingSelected");
  fireEvent.click(dark);
  expect(dark.className).toContain("onboardingSelected");
});

it("marks the selected preset", () => {
  const { getByText } = render(<Onboarding />);
  const minimal = getByText("Minimal").closest("button")!;
  fireEvent.click(minimal);
  expect(minimal.className).toContain("onboardingSelected");
});
