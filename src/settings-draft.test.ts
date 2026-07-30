import { expect, it } from "vitest";
import { isDirty, withoutSection, type MergedDraft } from "./settings-draft";
import type { SectionConfig } from "./useStats";

function draft(sections: Partial<SectionConfig>[]): MergedDraft {
  return { config: {} as MergedDraft["config"], profile: { sections } as MergedDraft["profile"], autostart: false };
}

it("removes the section with the matching instance", () => {
  const next = withoutSection(draft([{ id: "cpu", instance: "cpu-1" }, { id: "cpu", instance: "cpu-2" }]), "cpu-1");
  expect(next!.profile.sections.map((s) => s.instance)).toEqual(["cpu-2"]);
});

it("falls back to the id when a section has no instance", () => {
  const next = withoutSection(draft([{ id: "clock" }, { id: "date" }]), "clock");
  expect(next!.profile.sections.map((s) => s.id)).toEqual(["date"]);
});

it("leaves the draft alone when nothing matches", () => {
  const next = withoutSection(draft([{ id: "cpu", instance: "cpu-1" }]), "memory-1");
  expect(next!.profile.sections).toHaveLength(1);
});

it("keeps every other instance of the same widget id", () => {
  const next = withoutSection(draft([
    { id: "cpu", instance: "cpu-1" }, { id: "cpu", instance: "cpu-2" }, { id: "cpu", instance: "cpu-3" },
  ]), "cpu-2");
  expect(next!.profile.sections.map((s) => s.instance)).toEqual(["cpu-1", "cpu-3"]);
});

it("passes a null draft straight through", () => {
  expect(withoutSection(null, "cpu-1")).toBeNull();
});

function fullDraft(): MergedDraft {
  return {
    config: { opacity: 1, text_opacity: 1 } as MergedDraft["config"],
    profile: {
      sections: [{ id: "cpu", instance: "cpu", enabled: true, monitor: 0, x: 0, y: 0, width: 360 }],
      system_fields: [],
      show_cpu_cores: true,
      disks: [],
    } as unknown as MergedDraft["profile"],
    autostart: false,
  };
}

it("reports a fresh draft as clean", () => {
  expect(isDirty(fullDraft(), fullDraft())).toBe(false);
});

it("notices a changed config value", () => {
  const next = fullDraft();
  next.config = { ...next.config, opacity: 0.5 };
  expect(isDirty(next, fullDraft())).toBe(true);
});

it("notices a changed section width", () => {
  const next = fullDraft();
  next.profile = { ...next.profile, sections: [{ ...next.profile.sections[0], width: 420 }] };
  expect(isDirty(next, fullDraft())).toBe(true);
});

it("notices a flipped autostart toggle", () => {
  expect(isDirty({ ...fullDraft(), autostart: true }, fullDraft())).toBe(true);
});

it("ignores key insertion order so a spread does not read as a change", () => {
  const next = fullDraft();
  next.config = { text_opacity: 1, opacity: 1 } as MergedDraft["config"];
  expect(isDirty(next, fullDraft())).toBe(false);
});
