import { cleanup, fireEvent, render, within } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { MergedConfig } from "../useStats";
import { DatePreferences } from "./Date";

afterEach(cleanup);

function config(): MergedConfig {
  return {
    opacity: 1,
    sections: [
      { id: "date", instance: "date", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
      { id: "date", instance: "date-2", enabled: true, monitor: 0, x: 24, y: 24, width: 360 },
    ],
    system_fields: [],
    show_cpu_cores: true,
    disks: [],
  };
}

it("renders nothing when no date is enabled", () => {
  const bare = { ...config(), sections: [] };
  const { container } = render(<DatePreferences config={bare} onChange={vi.fn()} />);
  expect(container.firstChild).toBeNull();
});

it("edits weekday visibility only for the selected instance", () => {
  const onChange = vi.fn();
  const { getAllByRole } = render(<DatePreferences config={config()} onChange={onChange} />);
  fireEvent.click(getAllByRole("switch", { name: "Show weekday" })[1]);
  expect(onChange.mock.calls[0][0].sections[0].date_weekday).toBeUndefined();
  expect(onChange.mock.calls[0][0].sections[1].date_weekday).toBe(false);
});

it("edits the format only for the selected instance", () => {
  const onChange = vi.fn();
  const { getAllByRole } = render(<DatePreferences config={config()} onChange={onChange} />);
  fireEvent.click(getAllByRole("radio", { name: "Numeric" })[0]);
  expect(onChange.mock.calls[0][0].sections[0].date_format).toBe("numeric");
  expect(onChange.mock.calls[0][0].sections[1].date_format).toBeUndefined();
});

it("edits each date instance colour independently", () => {
  const onChange = vi.fn();
  const { getAllByLabelText } = render(<DatePreferences config={config()} onChange={onChange} />);
  fireEvent.change(getAllByLabelText("Color")[1], { target: { value: "#c07100" } });
  expect(onChange.mock.calls[0][0].sections[0].date_color).toBeUndefined();
  expect(onChange.mock.calls[0][0].sections[1].date_color).toBe("#c07100");
});

it("gives each date its own titled block", () => {
  const { getByRole } = render(<DatePreferences config={config()} onChange={vi.fn()} />);
  expect(getByRole("heading", { name: "Date", level: 3 })).toHaveAttribute("id", "instance-date");
  expect(getByRole("heading", { name: "Date 2", level: 3 })).toHaveAttribute("id", "instance-date-2");
});

it("associates repeated color controls with each date heading", () => {
  const { getByRole } = render(<DatePreferences config={config()} onChange={vi.fn()} />);
  expect(within(getByRole("group", { name: "Date" })).getByLabelText("Color")).not.toBeNull();
  expect(within(getByRole("group", { name: "Date 2" })).getByLabelText("Color")).not.toBeNull();
});
