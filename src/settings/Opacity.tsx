import {
  Label,
  Slider,
  SliderFill,
  SliderOutput,
  SliderThumb,
  SliderTrack,
} from "react-aria-components";

interface Props {
  label: string;
  value: number;
  onPreview: (value: number) => void;
  onCommit: (value: number) => void;
}

export function OpacityControl({ label, value, onPreview, onCommit }: Props) {
  return (
    <Slider
      className="settingsSlider"
      minValue={0.1}
      maxValue={1}
      step={0.01}
      value={value}
      onChange={onPreview}
      onChangeEnd={onCommit}
    >
      <Label className="settingsRowLabel">{label}</Label>
      <SliderOutput className="settingsValue">
        {({ state }) => `${Math.round(state.getThumbValue(0) * 100)}%`}
      </SliderOutput>
      <SliderTrack className="settingsSliderTrack">
        <SliderFill className="settingsSliderFill" />
        <SliderThumb className="settingsSliderThumb" />
      </SliderTrack>
    </Slider>
  );
}
