/// One list, two consumers: the sidebar renders it and the pane anchors to it.
/// A section listed here but not rendered leaves a dead nav link, so the two
/// are deliberately kept in one place.
export interface SettingsSectionMeta {
  id: string;
  label: string;
  group: "overlay" | "widgets";
}

export const SETTINGS_GROUPS: Array<{ id: SettingsSectionMeta["group"]; label: string }> = [
  { id: "overlay", label: "Overlay" },
  { id: "widgets", label: "Widgets" },
];

export const SETTINGS_SECTIONS: SettingsSectionMeta[] = [
  { id: "appearance", label: "Appearance", group: "overlay" },
  { id: "widgets", label: "Widgets", group: "overlay" },
  { id: "panels", label: "Panels", group: "overlay" },
  { id: "plugins", label: "Plugins", group: "overlay" },
  { id: "startup", label: "Startup", group: "overlay" },
  { id: "fields", label: "Fields", group: "widgets" },
  { id: "clock", label: "Clock", group: "widgets" },
  { id: "date", label: "Date", group: "widgets" },
  { id: "visualizer", label: "Visualizer", group: "widgets" },
  { id: "cpu", label: "CPU", group: "widgets" },
  { id: "memory", label: "Memory", group: "widgets" },
  { id: "disks", label: "Disks", group: "widgets" },
  { id: "network", label: "Network", group: "widgets" },
];
