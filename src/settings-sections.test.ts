import { expect, it } from "vitest";
import { settingsNavTree } from "./settings-sections";
import type { SectionConfig } from "./useStats";

function section(id: string, instance: string, enabled = true): SectionConfig {
  return { id, instance, enabled, monitor: 0, x: 0, y: 0, width: 360 };
}

function item(tree: ReturnType<typeof settingsNavTree>, id: string) {
  return tree.flatMap((group) => group.items).find((entry) => entry.id === id);
}

it("keeps static sections regardless of the configured widgets", () => {
  const tree = settingsNavTree([]);
  expect(item(tree, "appearance")?.children).toEqual([]);
  expect(item(tree, "network")?.children).toEqual([]);
});

it("hides a multi-instance parent that has no enabled instances", () => {
  const tree = settingsNavTree([section("cpu", "cpu")]);
  expect(item(tree, "clock")).toBeUndefined();
  expect(item(tree, "date")).toBeUndefined();
  expect(item(tree, "visualizer")).toBeUndefined();
});

it("hides a parent whose only instance is disabled", () => {
  const tree = settingsNavTree([section("clock", "clock", false)]);
  expect(item(tree, "clock")).toBeUndefined();
});

it("collects both visualizer kinds under one parent", () => {
  const tree = settingsNavTree([
    section("spectrum", "spectrum"),
    section("ring", "ring"),
    section("spectrum", "spectrum-2"),
  ]);
  expect(item(tree, "visualizer")?.children).toEqual([
    { anchorId: "instance-spectrum", label: "Spectrum" },
    { anchorId: "instance-ring", label: "Ring" },
    { anchorId: "instance-spectrum-2", label: "Spectrum 2" },
  ]);
});

it("prefixes child anchors so an instance never collides with a section id", () => {
  const tree = settingsNavTree([section("clock", "clock")]);
  expect(item(tree, "clock")?.id).toBe("clock");
  expect(item(tree, "clock")?.children[0].anchorId).toBe("instance-clock");
});

it("falls back to the section id when an instance has no name", () => {
  const nameless = { id: "date", enabled: true, monitor: 0, x: 0, y: 0, width: 360 } as SectionConfig;
  expect(item(settingsNavTree([nameless]), "date")?.children).toEqual([
    { anchorId: "instance-date", label: "Date" },
  ]);
});

it("keeps sections inside their declared group", () => {
  const tree = settingsNavTree([]);
  const overlay = tree.find((group) => group.id === "overlay");
  expect(overlay?.items.map((entry) => entry.id)).toEqual([
    "appearance", "widgets", "panels", "plugins", "startup",
  ]);
});
