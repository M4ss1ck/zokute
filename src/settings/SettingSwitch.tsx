import { Switch, type SwitchProps } from "react-aria-components";

interface Props {
  children: string;
  isSelected: boolean;
  onChange: SwitchProps["onChange"];
  isDisabled?: boolean;
}

export function SettingSwitch({ children, isSelected, onChange, isDisabled }: Props) {
  return (
    <Switch
      className="settingsSwitch"
      isSelected={isSelected}
      onChange={onChange}
      isDisabled={isDisabled}
    >
      <span className="settingsRowLabel">{children}</span>
      <span className="settingsSwitchTrack" aria-hidden="true">
        <span className="settingsSwitchThumb" />
      </span>
    </Switch>
  );
}
