import { IconGripVertical } from "@tabler/icons-react";
import {
  Button,
  GridList,
  GridListItem,
  useDragAndDrop,
} from "react-aria-components";
import type { StatsConfig, SystemField } from "../useStats";
import { HeaderToggle } from "./Header";
import { SettingSwitch } from "./SettingSwitch";

interface Props {
  available: SystemField[];
  config: StatsConfig;
  onChange: (next: StatsConfig) => void;
}

export function reorderSystemFields(
  order: string[],
  movingKeys: ReadonlySet<unknown>,
  targetKey: unknown,
  position: "before" | "after",
) {
  const moving = order.filter((id) => movingKeys.has(id));
  const remaining = order.filter((id) => !movingKeys.has(id));
  const targetIndex = remaining.indexOf(String(targetKey));
  if (!moving.length || targetIndex < 0) return order;
  const insertAt = targetIndex + (position === "after" ? 1 : 0);
  return [...remaining.slice(0, insertAt), ...moving, ...remaining.slice(insertAt)];
}

// fastfetch distinguishes repeated rows as "Display (LS27DG30X)" or "GPU 1";
// settings toggles the whole group, so the row-specific part is dropped.
function groupLabel(label: string) {
  return label.split(" (")[0].replace(/ \d+$/, "");
}

export function FieldToggles({ available, config, onChange }: Props) {
  const groups: SystemField[] = [];
  for (const field of available) {
    if (groups.some((group) => group.id === field.id)) continue;
    groups.push({ ...field, label: groupLabel(field.label) });
  }
  const selected = config.system_fields
    .map((id) => groups.find((group) => group.id === id))
    .filter((field): field is SystemField => field !== undefined);
  const hidden = groups.filter(
    (group) => !config.system_fields.includes(group.id),
  );
  const { dragAndDropHooks } = useDragAndDrop<SystemField>({
    getItems: (_keys, items) =>
      items.map((field) => ({ "text/plain": field.label })),
    onReorder(event) {
      if (event.target.dropPosition === "on") return;
      onChange({
        ...config,
        system_fields: reorderSystemFields(
          config.system_fields,
          event.keys,
          event.target.key,
          event.target.dropPosition,
        ),
      });
    },
  });
  function toggle(id: string, checked: boolean) {
    onChange({
      ...config,
      system_fields: checked
        ? [...config.system_fields, id]
        : config.system_fields.filter((field) => field !== id),
    });
  }
  return (
    <section className="settingsCard" aria-labelledby="system-title">
      <header className="settingsCardHeader">
        <h2 id="system-title">System</h2>
        <p>Drag visible fields to set their widget order.</p>
      </header>
      <div className="settingsCardBody">
        <HeaderToggle id="system" config={config} onChange={onChange} />
        <GridList
          className="settingsFieldList"
          aria-label="Visible system fields"
          items={selected}
          selectionMode="none"
          keyboardNavigationBehavior="tab"
          dragAndDropHooks={dragAndDropHooks}
        >
          {(field) => (
            <GridListItem className="settingsFieldRow" textValue={field.label}>
              <Button
                className="settingsDragButton"
                slot="drag"
                aria-label={`Reorder ${field.label}`}
              >
                <IconGripVertical aria-hidden="true" />
              </Button>
              <SettingSwitch
                isSelected
                onChange={(checked) => toggle(field.id, checked)}
              >
                {field.label}
              </SettingSwitch>
            </GridListItem>
          )}
        </GridList>
        {hidden.length > 0 ? (
          <div className="settingsHiddenFields" aria-label="Hidden system fields">
            <span className="settingsSubheading">Hidden fields</span>
            {hidden.map((field) => (
              <SettingSwitch
                key={field.id}
                isSelected={false}
                onChange={(checked) => toggle(field.id, checked)}
              >
                {field.label}
              </SettingSwitch>
            ))}
          </div>
        ) : null}
      </div>
    </section>
  );
}
