import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { MergedConfig } from "../useStats";
import { CpuPreferences } from "./Cpu";

afterEach(cleanup);

it("toggles the CPU cores preference", () => {
  const onChange = vi.fn();
  const config: MergedConfig = {
    opacity: 1,
    sections: [],
    system_fields: [],
    show_cpu_cores: true,
    disks: [],
  };
  const { getByRole } = render(
    <CpuPreferences config={config} onChange={onChange} />,
  );
  fireEvent.click(getByRole("switch", { name: "Show CPU cores" }));
  expect(onChange.mock.calls[0][0].show_cpu_cores).toBe(false);
});
