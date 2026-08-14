import { IconPlus, IconX } from "@tabler/icons-react";
import {
  Button,
  Label,
  Slider,
  SliderFill,
  SliderOutput,
  SliderThumb,
  SliderTrack,
} from "react-aria-components";
import { SettingSwitch } from "./SettingSwitch";
import type { MergedConfig, SectionConfig } from "../useStats";

// The kinds PanelWidget can actually render; spectrum/ring/plugin are absent
// there on purpose, so offering them would create invisible children.
const CHILD_KINDS = ["system", "cpu", "memory", "disk", "network", "clock", "date"] as const;

// Same shape as Sections.instanceLabel, but the taken-set also covers every
// child, because the Rust side uses instance as the window label.
function instanceLabel(id: string, sections: SectionConfig[]) {
  const labels = new Set(
    sections.flatMap((section) => [
      section.instance ?? section.id,
      ...(section.children ?? []).map((child) => child.instance ?? child.id),
    ]),
  );
  if (!labels.has(id)) return id;
  let suffix = 2;
  while (labels.has(`${id}-${suffix}`)) suffix += 1;
  return `${id}-${suffix}`;
}

function PanelSlider({ label, value, onChange }: { label: string; value: number; onChange: (value: number) => void }) {
  return (
    <Slider
      className="settingsSlider"
      minValue={0}
      maxValue={32}
      step={1}
      value={value}
      onChange={onChange}
    >
      <Label className="settingsRowLabel">{label}</Label>
      <SliderOutput className="settingsValue">
        {({ state }) => `${state.getThumbValue(0)}px`}
      </SliderOutput>
      <SliderTrack className="settingsSliderTrack">
        <SliderFill className="settingsSliderFill" />
        <SliderThumb className="settingsSliderThumb" />
      </SliderTrack>
    </Slider>
  );
}

interface Props {
  panel: SectionConfig;
  config: MergedConfig;
  onChange: (next: MergedConfig) => void;
}

export function PanelCard({ panel, config, onChange }: Props) {
  const instance = panel.instance ?? panel.id;
  const children = panel.children ?? [];
  function patchPanel(changes: Partial<SectionConfig>) {
    onChange({
      ...config,
      sections: config.sections.map((section) =>
        (section.instance ?? section.id) === instance ? { ...section, ...changes } : section,
      ),
    });
  }
  function removePanel() {
    onChange({
      ...config,
      sections: config.sections.filter((section) => (section.instance ?? section.id) !== instance),
    });
  }
  function addChild(kind: string) {
    const child: SectionConfig = {
      id: kind,
      instance: instanceLabel(kind, config.sections),
      enabled: true,
      show_header: true,
      monitor: 0,
      x: 0,
      y: 0,
      width: 360,
      scale: 1,
    };
    patchPanel({ children: [...children, child] });
  }
  function removeChild(childInstance: string) {
    patchPanel({ children: children.filter((child) => (child.instance ?? child.id) !== childInstance) });
  }
  return (
    <div className="settingsPanel" role="group" aria-labelledby={`panel-instance-${instance}`}>
      <div className="settingsPanelRow">
        <h3 className="settingsInstanceHeader" id={`panel-instance-${instance}`}>{instance}</h3>
        <Button aria-label={`Remove panel ${instance}`} className="settingsPanelRemove" onPress={removePanel}>
          <IconX aria-hidden="true" />
        </Button>
      </div>
      {children.length > 0 && (
        <div className="settingsPanelChildren">
          {children.map((child) => {
            const childInstance = child.instance ?? child.id;
            return (
              <div className="settingsPanelRow" key={childInstance}>
                <span className="settingsInstanceHeader">{childInstance}</span>
                <Button
                  aria-label={`Remove ${childInstance} from ${instance}`}
                  className="settingsPanelRemove"
                  onPress={() => removeChild(childInstance)}
                >
                  <IconX aria-hidden="true" />
                </Button>
              </div>
            );
          })}
        </div>
      )}
      <div className="settingsAddGrid settingsPanelAddGrid">
        {CHILD_KINDS.map((kind) => (
          <Button
            aria-label={`Add ${kind} to ${instance}`}
            className="settingsAddButton settingsPanelAddButton"
            key={kind}
            onPress={() => addChild(kind)}
          >
            <IconPlus aria-hidden="true" />
            <span>{kind}</span>
          </Button>
        ))}
      </div>
      <PanelSlider label="Gap" value={panel.panel_gap ?? 4} onChange={(panel_gap) => patchPanel({ panel_gap })} />
      <PanelSlider label="Padding" value={panel.panel_padding ?? 8} onChange={(panel_padding) => patchPanel({ panel_padding })} />
      <SettingSwitch
        isSelected={panel.panel_dividers ?? true}
        onChange={(panel_dividers) => patchPanel({ panel_dividers })}
      >
        Dividers
      </SettingSwitch>
    </div>
  );
}
