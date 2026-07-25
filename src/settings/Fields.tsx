import { IconGripVertical } from "@tabler/icons-react";
import {
  Button,
  GridList,
  GridListItem,
  useDragAndDrop,
} from "react-aria-components";
import type { StatsConfig, SystemField } from "../useStats";
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

export function FieldToggles({ available, config, onChange }: Props) {
  const selected = config.system_fields
    .map((id) => available.find((field) => field.id === id))
    .filter((field): field is SystemField => field !== undefined);
  const hidden = available.filter((field) => !config.system_fields.includes(field.id));
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
        <div className="settingsDivider" />
        <SettingSwitch
          isSelected={config.show_cpu_cores}
          onChange={(show_cpu_cores) => onChange({ ...config, show_cpu_cores })}
        >
          Show CPU cores
        </SettingSwitch>
      </div>
    </section>
  );
}
