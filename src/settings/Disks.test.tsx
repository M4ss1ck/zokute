import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { MergedConfig, Stats } from "../useStats";
import { DiskPreferences } from "./Disks";

afterEach(cleanup);

const detected: Stats["disks"] = [
  {
    id: "uuid-1",
    name: "nvme0n1p2",
    mount: "/",
    used_bytes: 0,
    total_bytes: 0,
    temperature_celsius: null,
    display_label: null,
  },
];

function config(): MergedConfig {
  return {
    opacity: 1,
    sections: [],
    system_fields: [],
    show_cpu_cores: true,
    disks: [
      { id: "uuid-1", enabled: true, label: null },
      { id: "uuid-gone", enabled: true, label: "Archive" },
    ],
  };
}

it("lists mounted disks by mount point and unmounted ones by their saved label", () => {
  const { getByLabelText } = render(
    <DiskPreferences detected={detected} config={config()} onChange={vi.fn()} />,
  );
  getByLabelText("Show /");
  getByLabelText("Show Archive");
});

it("toggles one disk without disturbing the other", () => {
  const onChange = vi.fn();
  const { getByLabelText } = render(
    <DiskPreferences detected={detected} config={config()} onChange={onChange} />,
  );
  fireEvent.click(getByLabelText("Show /"));
  expect(onChange.mock.calls[0][0].disks).toEqual([
    { id: "uuid-1", enabled: false, label: null },
    { id: "uuid-gone", enabled: true, label: "Archive" },
  ]);
});

it("stores a typed label and clears it back to null when emptied", () => {
  const onChange = vi.fn();
  const { getByLabelText } = render(
    <DiskPreferences detected={detected} config={config()} onChange={onChange} />,
  );
  const input = getByLabelText("Label for /");
  fireEvent.change(input, { target: { value: "System" } });
  expect(onChange.mock.calls[0][0].disks[0].label).toBe("System");
  fireEvent.change(input, { target: { value: "  " } });
  expect(onChange.mock.calls[1][0].disks[0].label).toBeNull();
});
