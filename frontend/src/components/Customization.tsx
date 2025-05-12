import { ComponentChildren } from "preact";
import { useState } from "preact/hooks";
import { Dropdown } from "./Dropdown";
import { CSSProperties } from "preact/compat";

const DEFAULT_FONT_RANGE = [100, 200, 300, 400, 500, 600, 700, 800, 900];

export const FONTS: Record<string, CSSProperties> = {
  Default: {
    fontFamily: "Consolas, sans-serif",
  },
  Roboto: {
    fontFamily: "Roboto, sans-serif",
    fontOpticalSizing: "auto",
    fontWeight: 500, // gotta replace this with something later
    fontStyle: "normal",
    fontVariationSettings: "wdth 100",
  },
  "Sans-Serif": {
    fontFamily: "sans-serif",
  },
  "Share Tech": {
    fontFamily: "Share Tech, sans-serif",
    fontWeight: 400,
    fontStyle: "normal",
  },
  "Nunito Sans": {
    fontFamily: "Nunito Sans, sans-serif",
    fontWeight: 500,
    fontStyle: "normal",
    fontVariationSettings: "wdth 100, YTLC 500"
  },
  "Ubuntu": {
    fontFamily: "Ubuntu, sans-serif",
    fontWeight: 500,
    fontStyle: "normal"
  }
};

const FONT_WEIGHTS: Record<keyof typeof FONTS, number[]> = {
  Default: DEFAULT_FONT_RANGE,
  Roboto: [100, 200, 300, 400, 500, 600, 700, 800, 900],
  "Sans-Serif": DEFAULT_FONT_RANGE,
  "Share Tech": [400],
  "Nunito Sans": DEFAULT_FONT_RANGE.map((v) => v + 100),
  Ubuntu: [300, 400, 500, 700],
  "": DEFAULT_FONT_RANGE
};

export const getFontWithWeight = (font: keyof typeof FONTS, weight: number) => {
  return {
    ...FONTS[font],
    fontWeight: weight
  }
};

export const Customization = (props: { id: string }) => {
  const [state, setState] = useState({
    // text
    upperText: "",
    lowerText: "",
    textColor: "",
    textSize: "",
    textFont: "",
    // border
    borderColor: "",
    borderSize: "",
    // animation for when the time changes
    animate: false,
  });

  return (
    <Fieldset legend="Timer Customization">
      <div className={"flex gap-2"}>
        <Dropdown
          placeholder="Font"
          values={Object.keys(FONTS)}
          callback={() => {}}
          valuesStyle={Object.values(FONTS)}
        />
        <Dropdown 
          placeholder="Font Weight"
          values={FONT_WEIGHTS[state.textFont].map(String)}
          callback={() => {}}
        />

      </div>
    </Fieldset>
  );
};

export const Fieldset = (props: {
  legend: string;
  legendTitle?: string;
  className?: string;
  children?: ComponentChildren;
}) => {
  return (
    <fieldset
      title={props.legendTitle}
      className={
        "border-1 border-gray-700 rounded-lg p-2 pb-3 " + props.className
      }
    >
      <legend>{props.legend}</legend>
      {props.children}
    </fieldset>
  );
};
