/// One list, two consumers: the sidebar renders it and the pane anchors to it.
/// A section listed here but not rendered leaves a dead nav link, so the two
/// are deliberately kept in one place.
import type { SectionConfig } from "./useStats";
import { instanceTitle } from "./instance-title";

export interface SettingsSectionMeta {
  id: string;
  label: string;
  group: "overlay" | "widgets";
  /// Section ids whose instances become children of this entry. Absent means
  /// the entry is static and always renders.
  kinds?: string[];
}

export const SETTINGS_GROUPS: Array<{ id: SettingsSectionMeta["group"]; label: string }> = [
  { id: "overlay", label: "Overlay" },
  { id: "widgets", label: "Widgets" },
];

export const SETTINGS_SECTIONS: SettingsSectionMeta[] = [
  { id: "appearance", label: "Appearance", group: "overlay" },
  { id: "widgets", label: "Visible Widgets", group: "overlay" },
  { id: "panels", label: "Panels", group: "overlay" },
  { id: "plugins", label: "Plugins", group: "overlay" },
  { id: "startup", label: "Startup", group: "overlay" },
  { id: "fields", label: "Fields", group: "widgets" },
  { id: "clock", label: "Clock", group: "widgets", kinds: ["clock"] },
  { id: "date", label: "Date", group: "widgets", kinds: ["date"] },
  { id: "visualizer", label: "Visualizer", group: "widgets", kinds: ["spectrum", "ring"] },
  { id: "cpu", label: "CPU", group: "widgets" },
  { id: "memory", label: "Memory", group: "widgets" },
  { id: "disks", label: "Disks", group: "widgets" },
  { id: "network", label: "Network", group: "widgets" },
];

export interface SettingsNavChild {
  anchorId: string;
  label: string;
}

export interface SettingsNavItem {
  id: string;
  label: string;
  children: SettingsNavChild[];
}

export interface SettingsNavGroup {
  id: SettingsSectionMeta["group"];
  label: string;
  items: SettingsNavItem[];
}

function navChildren(meta: SettingsSectionMeta, sections: SectionConfig[]): SettingsNavChild[] {
  const kinds = meta.kinds;
  if (!kinds) return [];
  return sections
    .filter((section) => section.enabled && kinds.includes(section.id))
    .map((section) => {
      const instance = section.instance ?? section.id;
      return { anchorId: `instance-${instance}`, label: instanceTitle(instance) };
    });
}

/// A card with no instances renders nothing, so its nav entry would scroll
/// nowhere. Dropping the entry is what keeps the sidebar honest.
export function settingsNavTree(sections: SectionConfig[]): SettingsNavGroup[] {
  return SETTINGS_GROUPS.map((group) => ({
    id: group.id,
    label: group.label,
    items: SETTINGS_SECTIONS
      .filter((meta) => meta.group === group.id)
      .map((meta) => ({ meta, children: navChildren(meta, sections) }))
      .filter(({ meta, children }) => !meta.kinds || children.length > 0)
      .map(({ meta, children }) => ({ id: meta.id, label: meta.label, children })),
  }));
}
