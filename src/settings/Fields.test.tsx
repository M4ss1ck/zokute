import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { MergedConfig, SystemField } from "../useStats";
import { FieldToggles, reorderSystemFields } from "./Fields";

afterEach(cleanup);

const available: SystemField[] = [
  { id: "os", label: "OS", value: "Linux" },
  { id: "kernel", label: "Kernel", value: "6.17" },
];

function config(): MergedConfig {
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

it("lists a repeated field once, under a group label", () => {
  const available = [
    { id: "display", label: "Display (A)", value: "1920x1080" },
    { id: "display", label: "Display (B)", value: "2560x1440" },
    { id: "gpu", label: "GPU 1", value: "Radeon" },
    { id: "gpu", label: "GPU 2", value: "780M" },
  ];
  const selected = { ...config(), system_fields: ["display"] };
  const onChange = vi.fn();
  const { getAllByText, getByText } = render(
    <FieldToggles
      available={available}
      config={selected}
      onChange={onChange}
    />,
  );

  expect(getAllByText("Display")).toHaveLength(1);
  expect(getByText("GPU")).toBeTruthy();
});

it("hides every row of a group when its toggle is switched off", () => {
  const available = [
    { id: "display", label: "Display (A)", value: "1920x1080" },
    { id: "display", label: "Display (B)", value: "2560x1440" },
  ];
  const selected = { ...config(), system_fields: ["display"] };
  const onChange = vi.fn();
  const { getByRole } = render(
    <FieldToggles
      available={available}
      config={selected}
      onChange={onChange}
    />,
  );

  fireEvent.click(getByRole("switch", { name: "Display" }));

  expect(onChange.mock.calls[0][0].system_fields).toEqual([]);
});
