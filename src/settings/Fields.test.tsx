import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { StatsConfig, SystemField } from "../useStats";
import { FieldToggles } from "./Fields";

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
  const { getByLabelText, queryByLabelText } = render(
    <FieldToggles available={available} config={config()} onChange={vi.fn()} />,
  );
  expect((getByLabelText("OS") as HTMLInputElement).checked).toBe(true);
  expect((getByLabelText("Kernel") as HTMLInputElement).checked).toBe(false);
  expect(queryByLabelText("Packages")).toBeNull();
});

it("adds a field in the backend's own order rather than click order", () => {
  const onChange = vi.fn();
  const { getByLabelText } = render(
    <FieldToggles available={available} config={config()} onChange={onChange} />,
  );
  fireEvent.click(getByLabelText("Kernel"));
  expect(onChange.mock.calls[0][0].system_fields).toEqual(["os", "kernel"]);
});

it("removes a field without disturbing the others", () => {
  const onChange = vi.fn();
  const selected = { ...config(), system_fields: ["os", "kernel"] };
  const { getByLabelText } = render(
    <FieldToggles available={available} config={selected} onChange={onChange} />,
  );
  fireEvent.click(getByLabelText("OS"));
  expect(onChange.mock.calls[0][0].system_fields).toEqual(["kernel"]);
});

it("toggles the CPU cores preference", () => {
  const onChange = vi.fn();
  const { getByLabelText } = render(
    <FieldToggles available={available} config={config()} onChange={onChange} />,
  );
  fireEvent.click(getByLabelText("Show CPU cores"));
  expect(onChange.mock.calls[0][0].show_cpu_cores).toBe(false);
});
