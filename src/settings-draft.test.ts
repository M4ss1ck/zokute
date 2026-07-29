import { expect, it } from "vitest";
import { withoutSection, type MergedDraft } from "./settings-draft";
import type { SectionConfig } from "./useStats";

function draft(sections: Partial<SectionConfig>[]): MergedDraft {
  return { config: {} as MergedDraft["config"], profile: { sections } as MergedDraft["profile"] };
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
