import { fireEvent, render, waitFor } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { LayoutEditor } from "./LayoutEditor";
import type { Stats } from "./useStats";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn(() => Promise.resolve()) }));
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
}));

const stats = {
  profile: {
    sections: [
      { id: "system", instance: "system", enabled: true, x: 12, y: 24, width: 360 },
      { id: "cpu", instance: "cpu-2", enabled: true, x: 48, y: 96, width: 420 },
    ],
  },
} as unknown as Stats;

it("offers every widget as an explicit layout-editor choice", async () => {
  const { getByRole } = render(<LayoutEditor stats={stats} />);
  const system = getByRole("button", { name: "system" });
  const cpu = getByRole("button", { name: "cpu-2 (cpu)" });

  await waitFor(() => expect(system).toHaveAttribute("aria-pressed", "true"));
  fireEvent.click(cpu);

  expect(cpu).toHaveAttribute("aria-pressed", "true");
  expect(system).toHaveAttribute("aria-pressed", "false");
});
