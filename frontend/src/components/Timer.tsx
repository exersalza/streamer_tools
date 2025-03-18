import { useEffect, useReducer, useRef, useState } from "preact/hooks";
import { TimerType } from "../main";
import { Icons } from "./Icons";
import { ChangeEvent } from "preact/compat";
import { API, parseTime } from "./utils";
import { HexColorPicker, HexColorInput } from "powerful-color-picker";
import { Loading } from "./Loading";

type States = {};

interface TimerButtonProps {
  data: TimerType;
  active: string;
  setActiveTimer: (timer: string) => void;
}

export function TimerButton(props: TimerButtonProps) {
  const [state, setState] = useState<States>({});

  const onClickHandler = (e: MouseEvent) => {
    props.setActiveTimer(props.data.id);
  };

  return (
    <button
      key={props.data.id}
      className={`cursor-pointer select-none rounded transition-all h-8 ${props.active === props.data.id ? "bg-gray-800 hover:bg-gray-600" : "bg-gray-800 hover:bg-gray-700"} min-w-40 border-1 border-gray-700`}
      onClick={onClickHandler}
    >
      <p className={"text-zinc-100 flex gap-2 px-1"}>
        <span
          style={{
            color:
              props.data.color !== "#000000"
                ? props.data.color
                : "var(--color-zinc-100)",
          }}
        >
          {Icons.clock}
        </span>{" "}
        {props.data.name}
      </p>
    </button>
  );
}

interface TimerProps {
  uuid: string;
}

type TimerStates = {
  loading: boolean;
  data: TimerType | null;
};

export function Timer(props: TimerProps) {
  const [state, setState] = useState<TimerStates>({
    loading: true,
    data: null,
  });

  useEffect(() => {
    setState(() => ({ loading: true, data: null }));

    fetch(API + `/get_timer?uuid=${props.uuid}`).then(async (res) => {
      if (!res.ok) {
        console.error(await res.text());
        return;
      }

      const data = await res.json();
      setState(() => ({ loading: false, data: data[0] }));
    });
  }, [props.uuid]);

  const buttonOnClick = (e: MouseEvent) => {
    const targetId = (e.target as HTMLButtonElement).id.split("-")[2];

    fetch(API + "/post_button_pressed", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        function: targetId,
      }),
    }).then(async (res) => {
      if (!res.ok) {
        console.error(await res.text());
        return;
      }
    });
  };

  if (state.loading) {
    return (
      <div className={"h-full w-full p-2 text-zinc-100"}>
        <div className={"flex place-items-center gap-2"}>
          <p>Loading</p>
          {Icons.loading}
        </div>
      </div>
    );
  }

  return (
    <div className={"h-full w-full text-zinc-100 p-2"}>
      <p className={"font-bold text-2xl"}>{state.data?.name}</p>
      <p className={"font-semibold text-zinc-400"}>Uuid {props.uuid}</p>
      <div className={"flex flex-col gap-4 mt-8"}>
        <div>
          <p className={"text-zinc-100 font-semibold text-xl select-none"}>
            Control elements
          </p>
          <div className={"flex gap-2 "}>
            <button
              id={"control-button-M5"}
              onClick={buttonOnClick}
              className={
                "transition-all text-zinc-400 hover:text-zinc-100 rounded-lg bg-gray-800 p-2 cursor-pointer border-1 border-gray-700"
              }
            >
              -5 Min
            </button>
            <button
              id={"control-button-M1"}
              onClick={buttonOnClick}
              className={
                "transition-all  text-zinc-400 hover:text-zinc-100 rounded-lg bg-gray-800 p-2 cursor-pointer border-1 border-gray-600"
              }
            >
              -1 Min
            </button>

            <button
              id={"control-button-Stop"}
              onClick={buttonOnClick}
              className={
                "transition-all text-zinc-400 rounded-lg bg-gray-800 p-2 cursor-pointer border-1 border-gray-600 hover:text-red-500"
              }
            >
              {Icons.stop}
            </button>

            <button
              id={"control-button-Play"}
              onClick={buttonOnClick}
              className={
                "transition-all text-zinc-400 rounded-lg bg-gray-800 p-2 cursor-pointer border-1 border-gray-600 hover:text-green-500"
              }
            >
              {Icons.play}
            </button>
            <button
              id={"control-button-Pause"}
              onClick={buttonOnClick}
              className={
                "transition-all text-zinc-400 hover:text-zinc-100 transition-all rounded-lg bg-gray-800 p-2 cursor-pointer border-1 border-gray-600 "
              }
            >
              {Icons.pombear}
            </button>

            <button
              id={"control-button-P1"}
              onClick={buttonOnClick}
              className={
                "transition-all text-zinc-400 hover:text-zinc-100 rounded-lg bg-gray-800 p-2 cursor-pointer border-1 border-gray-600"
              }
            >
              +1 Min
            </button>
            <button
              id={"control-button-P5"}
              onClick={buttonOnClick}
              className={
                "transition-all text-zinc-400 hover:text-zinc-100 rounded-lg bg-gray-800 p-2 cursor-pointer border-1 border-gray-600"
              }
            >
              +5 Min
            </button>
          </div>
        </div>
        <div className={""}>
          <iframe
            src={`/${props.uuid}`}
            className={"rounded-lg ring-gray-700 ring-1"}
          />
        </div>
      </div>
    </div>
  );
}

interface TimerOverlayProps {
  hidden: boolean;
  hideWindow: () => void;
  update: (prev: number) => void;
}

const OVERLAY_STATES_DEFAULT = {
  main: false,
  is_active: false,
  color: "#000000",
  hour: 0,
  minute: 0,
  second: 0,
  follow: 0,
  sub_t1: 0,
  sub_t2: 0,
  sub_t3: 0,
  bits_each_n: 0,
  bits_n: 0,
  dono_each_n: 0,
  dono_n: 0,
};

export function CreateTimerOverlay(props: TimerOverlayProps) {
  const [values, setValues] = useState(OVERLAY_STATES_DEFAULT);

  const [showColorPick, setShowColorPick] = useState(false);
  const [color, setColor] = useState("#aabbcc");

  const blockInvalidChar = (e: KeyboardEvent) =>
    ["e", "E", "+", "-"].includes(e.key) && e.preventDefault();
  const blockInvalidHex = (e: KeyboardEvent) =>
    !/[\#a-fA-F0-9]/.test(e.key) && e.preventDefault();

  const value_thing = {
    // change titles
    follow: ["Follow", ""],
    sub_t1: ["Tier 1 Sub", ""],
    sub_t2: ["Tier 2 Sub", ""],
    sub_t3: ["Tier 3 Sub", ""],
    bits_each_n: ["(Bits) X amount spent", "border-yellow-700"],
    bits_n: ["(Bits) Increase time by", "border-yellow-700"],
    dono_each_n: ["(Dono) X amount spent", "border-green-700"],
    dono_n: ["(Dono) Increase time by", "border-green-700"],
  };

  let name = useRef<HTMLInputElement>();
  const container = useRef<HTMLDivElement>(null);

  function changeColor(e: InputEvent) {
    let intar = (e.target as HTMLInputElement).value ?? "";

    if (!/^#?[a-fA-F0-9]{0,6}$/gm.test(intar)) return;

    setColor(intar);
  }

  function minMax(e: InputEvent) {
    const target = e.target as HTMLInputElement;
    const targetId = target.id.split("-")[3];
    setValues((prev) => ({ ...prev, [targetId]: target.valueAsNumber }));

    if (targetId === "hour") {
      if (target.value.length > 6) {
        setValues((prev) => ({ ...prev, hour: target.value.slice(0, 6) }));
      }
    }

    if (targetId === "minute" || targetId === "second") {
      if (target.value.length > 2) {
        setValues((prev) => ({
          ...prev,
          [targetId]: target.value.slice(0, 2),
        }));
      }
    }

    if (target.valueAsNumber < 0) {
      setValues((prev) => ({ ...prev, [targetId]: 0 }));
    }

    if (targetId === "minute" || targetId === "second") {
      if (target.valueAsNumber >= 59) {
        setValues((prev) => ({ ...prev, [targetId]: 59 }));
      }
    }
  }

  useEffect(() => {
    if (!props.hidden) {
    }
  }, [props.hidden]);

  function createTimer() {
    fetch(API + "/post_create_timer", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: crypto.randomUUID(),
        name: name.current?.value,
        main: values.main,
        timer: values.hour * 60 * 60 + values.minute * 60 + values.second,
        color: color,
        is_active: values.is_active,
        overtitle: "",
        undertitle: "",
        increase_times: {
          follow: values.follow,
          sub_t1: values.sub_t1,
          sub_t2: values.sub_t2,
          sub_t3: values.sub_t3,
          bits_each_n: values.bits_each_n,
          bits_n: values.bits_n,
          dono_each_n: values.dono_each_n,
          dono_n: values.dono_n,
        },
      }),
    }).then(async (res) => {
      if (!res.ok) console.log(await res.text());
      props.update(Math.random());
      props.hideWindow();
      setValues(OVERLAY_STATES_DEFAULT);
      name.current.value = ""; // SHUT THE FUCK UP LSP
      setColor("#000000");
    });
  }

  function handleClick(evt: React.MouseEvent<HTMLElement>) {
    if (container.current && !container.current.contains(evt.target)) {
      props.hideWindow();
    }
  }

  return (
    <div
      className={`${props.hidden ? "hidden" : ""} transition-all h-screen w-screen absolute backdrop-blur-xs z-50 grid place-content-center`}
      onClick={handleClick}
    >
      <div
        ref={container}
        className={
          "h-190 w-80 bg-gray-900 rounded-lg flex-col border-1 border-gray-800 p-2"
        }
      >
        <p
          className={"text-zinc-100 text-lg flex flex-col place-items-center "}
        >
          Create a timer
        </p>
        <div className={"flex flex-col gap-2"}>
          <label for={"timer-creation-name"} className={"text-zinc-100"}>
            Timer name
          </label>
          <input
            id={"timer-creation-name"}
            placeholder={"what-a-timer"}
            ref={name}
            className={"border-1 border-gray-700 rounded text-zinc-100 p-2"}
            maxlength={50}
          />

          <label for={"timer-creation-times"} className={"text-zinc-100"}>
            Initial time
          </label>
          <div className={"flex place-items-baseline gap-2"}>
            <input
              id={"timer-creation-times-hour"}
              placeholder={"Hrs..."}
              className={
                "border-1 w-25 border-gray-700 rounded text-zinc-100 p-2"
              }
              type="number"
              onKeyDown={blockInvalidChar}
              onInput={minMax}
              value={values.hour}
            />

            <p className={"text-zinc-100 text-2xl"}>:</p>
            <input
              id={"timer-creation-times-minute"}
              placeholder={"Min..."}
              className={
                "border-1 w-20 border-gray-700 rounded text-zinc-100 p-2"
              }
              type="number"
              onKeyDown={blockInvalidChar}
              onInput={minMax}
              value={values.minute}
            />

            <p className={"text-zinc-100 text-2xl"}>:</p>
            <input
              id={"timer-creation-times-second"}
              placeholder={"Sec..."}
              className={
                "border-1 w-20 border-gray-700 rounded text-zinc-100 p-2"
              }
              type="number"
              onKeyDown={blockInvalidChar}
              onInput={minMax}
              value={values.second}
            />
          </div>
          <div className={"flex flex-col gap-2"}>
            {Object.keys(value_thing).map((key) => {
              return (
                <div className={"flex place-items-center gap-2"}>
                  <input
                    id={`timer-creation-increases-${key}`}
                    value={values[key]}
                    className={`border-1 w-20 border-gray-700 rounded text-zinc-100 p-2 ${value_thing[key][1]}`}
                    type="number"
                    onKeyDown={blockInvalidChar}
                    onInput={minMax}
                  />
                  <p className={`text-zinc-100 }`}>{value_thing[key][0]}</p>
                </div>
              );
            })}
          </div>
        </div>
        <div>
          <div className={"flex flex-col gap-2"}>
            <div
              className={"flex gap-2"}
              title={
                "Is this the main subathon timer (does subs, and bits increase this timer or not)"
              }
            >
              <input
                type="checkbox"
                id={"timer-creation-main"}
                checked={values.main}
                onChange={() => {
                  setValues((prev) => ({ ...prev, main: !prev.main }));
                }}
              />
              <label
                for={"timer-creation-main"}
                className={"text-zinc-100 select-none"}
              >
                Mark as Main timer
              </label>
            </div>
            <div
              className={"flex gap-2"}
              title={"Should the timer be set active upon creation"}
            >
              <input
                type="checkbox"
                id={"timer-creation-is_active"}
                checked={values.is_active}
                onChange={() => {
                  setValues((prev) => ({
                    ...prev,
                    is_active: !prev.is_active,
                  }));
                }}
              />
              <label
                for={"timer-creation-is_active"}
                className={"text-zinc-100 select-none"}
              >
                Is timer active
              </label>
            </div>
            <div
              className={`absolute -translate-y-50 ${showColorPick ? "" : "hidden"} p-1 rounded-lg bg-gray-800 flex flex-col gap-2`}
            >
              <HexColorPicker color={color} onChange={setColor} />
              <input
                className={"px-1 ring-1 ring-zinc-500 rounded text-zinc-100"}
                value={color}
                onInput={changeColor}
              />
              <button
                className={
                  "text-zinc-100 cursor-pointer hover:bg-gray-600 rounded"
                }
                onClick={() => {
                  setShowColorPick(false);
                }}
              >
                Save
              </button>
            </div>
            <button
              className={"text-zinc-100 cursor-pointer"}
              onClick={() => {
                setShowColorPick((prev) => !prev);
              }}
            >
              Spawn color picker
            </button>
          </div>
        </div>
        <div className={"flex w-full gap-2 mt-3"}>
          <button
            onClick={createTimer}
            className={`cursor-pointer text-zinc-100 rounded transition-all h-8 w-full bg-gray-800 hover:bg-gray-600`}
          >
            Create
          </button>
          <button
            onClick={props.hideWindow}
            className={`cursor-pointer text-zinc-100 rounded transition-all h-8 w-full bg-gray-800 hover:bg-gray-600`}
          >
            Abort
          </button>
        </div>
      </div>
    </div>
  );
}

interface NumberProps {
  number: number | string;
  showAnimation?: boolean;
}

const NumberThingy = (props: NumberProps) => {
  return (
    <div className={"relative"}>
      <div className={"overflow-hidden h-16 text-6xl font-mono"}>
        {Array.from({ length: 10 }, (_, i) => i).map((v) => {
          return (
            <p
              className={`${!!props.showAnimation ? "transition" : ""} select-none`}
              style={{ transform: `translateY(calc(-60px * ${props.number}))` }}
            >
              {v}
            </p>
          );
        })}
      </div>
    </div>
  );
};

const Countdown = (props: {
  sec: number;
  className?: string;
  showAnimation?: boolean;
}) => {
  const [time, setTime] = useState("");

  const convertTimeToString = (s: number) => {
    const hours = Math.floor(s / 3600);
    const minutes = Math.floor((s % 3600) / 60);
    const seconds = s % 60;

    return (
      String(hours).padStart(3, "0") +
      String(minutes).padStart(2, "0") +
      String(seconds).padStart(2, "0")
    );
  };

  useEffect(() => {
    setTime(convertTimeToString(props.sec));
  }, [props.sec]);

  return (
    <div className={`flex ${props.className}`}>
      <NumberThingy number={time[0]} showAnimation={props.showAnimation} />
      <NumberThingy number={time[1]} showAnimation={props.showAnimation} />
      <NumberThingy number={time[2]} showAnimation={props.showAnimation} />
      <p className={"text-6xl h-16 font-mono"}>:</p>
      <NumberThingy number={time[3]} showAnimation={props.showAnimation} />
      <NumberThingy number={time[4]} showAnimation={props.showAnimation} />
      <p className={"text-6xl h-16 font-mono"}>:</p>
      <NumberThingy number={time[5]} showAnimation={props.showAnimation} />
      <NumberThingy number={time[6]} showAnimation={props.showAnimation} />
    </div>
  );
};

type TimerCompState = {
  time: number;
  loading: boolean;
  timerPaused: boolean;
  showAnimation: boolean;
  text: {
    upper: string;
    lower: string;
  };
  custom: {
    text: {
      upper: string;
      lower: string;
    };
  };
};

type Actions = {
  type:
    | "UpdateTime"
    | "IncTime"
    | "DecTime"
    | "TogglePause"
    | "UpdateText"
    | "ToggleAnimate"
    | "ToggleLoading";
  payload?: any;
};

export const TimerWidget = () => {
  const reducer = (prev: TimerCompState, action: Actions) => {
    switch (action.type) {
      case "UpdateTime":
        return { ...prev, time: action.payload.time };
      case "IncTime":
        return { ...prev, time: prev.time + 1 };
      case "DecTime":
        return { ...prev, time: Math.max(0, prev.time - 1) };
      case "UpdateText":
        return {
          ...prev,
          text: { ...prev.text, [action.payload.key]: action.payload.value },
        };
      case "TogglePause":
        return {
          ...prev,
          timerPaused: !prev.timerPaused,
        };
      case "ToggleAnimate":
        return {
          ...prev,
          showAnimation: !prev.showAnimation,
        };
      case "ToggleLoading":
        return {
          ...prev,
          loading: !prev.loading,
        };
      default:
        throw Error("Action is not valid");
    }
  };

  const [state, dispatch] = useReducer(reducer, {
    time: 0,
    loading: false,
    text: { upper: "cool text", lower: "cool text" },
    showAnimation: true,
    timerPaused: false,
    custom: {
      text: {
        upper: "",
        lower: "",
      },
    },
  });

  useEffect(() => {
    fetch(API + `/get_timer?uuid=${location.pathname.replace("/", "")}`).then(
      async (res) => {
        if (!res.ok) {
          console.error(await res.text());
          return;
        }

        const d = await res.json();
        console.log(d)
      },
    );
  }, []);

  if (state.loading) {
    return <Loading />;
  }

  return (
    <div className={"h-screen w-screen text-white flex flex-col items-center"}>
      <Countdown sec={state.time} />
    </div>
  );
};
