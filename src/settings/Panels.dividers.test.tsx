import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { MergedConfig, SectionConfig } from "../useStats";
import { PanelPreferences } from "./Panels";

// This project does not set vitest `globals`, so Testing Library never
// registers its automatic cleanup and renders would otherwise accumulate.
afterEach(cleanup);

function config(sections: SectionConfig[]): MergedConfig {
  return {
    opacity: 1,
    sections,
    system_fields: [],
    show_cpu_cores: true,
    disks: [],
  };
}

function panel(overrides: Partial<SectionConfig> = {}): SectionConfig {
  return {
    id: "panel",
    instance: "panel",
    enabled: true,
    show_header: true,
    monitor: 0,
    x: 24,
    y: 24,
    width: 360,
    scale: 1,
    children: [],
    panel_gap: 4,
    panel_padding: 8,
    ...overrides,
  };
}

function child(id: string, instance: string): SectionConfig {
  return { id, instance, enabled: true, show_header: true, monitor: 0, x: 0, y: 0, width: 360, scale: 1 };
}

it("toggling the Dividers switch writes panel_dividers false and leaves children and other fields untouched", () => {
  const onChange = vi.fn();
  const { getByRole } = render(
    <PanelPreferences config={config([panel({ children: [child("cpu", "cpu")] })])} onChange={onChange} />,
  );
  fireEvent.click(getByRole("switch", { name: "Dividers" }));
  const sections = onChange.mock.calls[0][0].sections;
  expect(sections[0]).toEqual({ ...panel({ children: [child("cpu", "cpu")] }), panel_dividers: false });
});
