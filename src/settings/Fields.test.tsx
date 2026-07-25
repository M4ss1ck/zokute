import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { StatsConfig, SystemField } from "../useStats";
import { FieldToggles, reorderSystemFields } from "./Fields";

afterEach(cleanup);

const available: SystemField[] = [
  { id: "os", label: "OS", value: "Linux" },
  { id: "kernel", label: "Kernel", value: "6.17" },
];

function config(): StatsConfig {
  return {
    opacity: 1,
    sections: [],
    system_fields: ["os"],
    show_cpu_cores: true,
    disks: [],
  };
}

it("offers only fields the backend reported", () => {
  const { getByRole, queryByRole } = render(
    <FieldToggles available={available} config={config()} onChange={vi.fn()} />,
  );
  expect((getByRole("switch", { name: "OS" }) as HTMLInputElement).checked).toBe(true);
  expect((getByRole("switch", { name: "Kernel" }) as HTMLInputElement).checked).toBe(false);
  expect(queryByRole("switch", { name: "Packages" })).toBeNull();
});

it("appends a newly visible field to the widget order", () => {
  const onChange = vi.fn();
  const { getByRole } = render(
    <FieldToggles available={available} config={config()} onChange={onChange} />,
  );
  fireEvent.click(getByRole("switch", { name: "Kernel" }));
  expect(onChange.mock.calls[0][0].system_fields).toEqual(["os", "kernel"]);
});

it("removes a field without disturbing the others", () => {
  const onChange = vi.fn();
  const selected = { ...config(), system_fields: ["os", "kernel"] };
  const { getByRole } = render(
    <FieldToggles available={available} config={selected} onChange={onChange} />,
  );
  fireEvent.click(getByRole("switch", { name: "OS" }));
  expect(onChange.mock.calls[0][0].system_fields).toEqual(["kernel"]);
});

it("reorders visible fields around the drop target", () => {
  expect(
    reorderSystemFields(
      ["os", "host", "kernel", "uptime"],
      new Set(["uptime"]),
      "host",
      "before",
    ),
  ).toEqual(["os", "uptime", "host", "kernel"]);
});
