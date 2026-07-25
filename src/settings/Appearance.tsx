import type { StatsConfig } from "../useStats";
import { ColorControl } from "./Color";
import { OpacityControl } from "./Opacity";
import { SettingSwitch } from "./SettingSwitch";

interface Props {
  config: StatsConfig;
  backgroundPreview: number | null;
  textPreview: number | null;
  onBackgroundPreview: (value: number) => void;
  onBackgroundCommit: (value: number) => void;
  onTextPreview: (value: number) => void;
  onTextCommit: (value: number) => void;
  onChange: (next: StatsConfig) => void;
}

export function Appearance({
  config,
  backgroundPreview,
  textPreview,
  onBackgroundPreview,
  onBackgroundCommit,
  onTextPreview,
  onTextCommit,
  onChange,
}: Props) {
  return (
    <section className="settingsCard" aria-labelledby="appearance-title">
      <header className="settingsCardHeader">
        <h2 id="appearance-title">Appearance</h2>
        <p>Control the overlay surface and visual emphasis.</p>
      </header>
      <div className="settingsCardBody">
        <SettingSwitch
          isSelected={config.show_background ?? true}
          onChange={(show_background) => onChange({ ...config, show_background })}
        >
          Show background
        </SettingSwitch>
        <OpacityControl
          label="Background opacity"
          value={backgroundPreview ?? config.opacity}
          onPreview={onBackgroundPreview}
          onCommit={onBackgroundCommit}
        />
        <OpacityControl
          label="Text opacity"
          value={textPreview ?? config.text_opacity ?? 1}
          onPreview={onTextPreview}
          onCommit={onTextCommit}
        />
        <ColorControl
          label="Text color"
          value={config.text_color ?? "#292824"}
          onChange={(text_color) => onChange({ ...config, text_color })}
        />
        <ColorControl
          label="Graph color"
          value={config.graph_color ?? "#494137"}
          onChange={(graph_color) => onChange({ ...config, graph_color })}
        />
        <ColorControl
          label="Icon color"
          value={config.icon_color ?? "#c07100"}
          onChange={(icon_color) => onChange({ ...config, icon_color })}
        />
      </div>
    </section>
  );
}
