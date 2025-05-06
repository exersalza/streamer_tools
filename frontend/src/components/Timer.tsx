import { useEffect, useReducer, useRef, useState } from "preact/hooks";
import { TimerType } from "../main";
import { Icons } from "./Icons";
import { _T, API, BACKEND, HOST, PORT, RECONNECT_AFTER } from "./utils";
import { HexColorPicker } from "powerful-color-picker";
import { Loading } from "./Loading";
import { ClipboardCopy, Pause, Play, Square } from "lucide-preact";
import { BUTTON_THEME, MainButton } from "./Buttons";
import { RefObject } from "preact";
import { setupRerender } from "preact/test-utils";

type States = {};

let socket: WebSocket | null = null;

export const getId = () => {
  return location.pathname.replace("/", "");
};

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
          {["bmc"].includes(props.data.name) ? (
            <img
              className={"size-6 rounded-full"}
              src="https://cdn.7tv.app/emote/01GY6Y9T6G000CX7G8QSR9TRP1/2x.avif"
            />
          ) : (
            Icons.clock
          )}
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
  background_white: boolean;
};

// the timer comp that is shown when you press a timer on the dashboard
export function Timer(props: TimerProps) {
  const [state, setState] = useState<TimerStates>({
    loading: true,
    data: null,
    background_white: false,
  });

  useEffect(() => {
    setState((prev) => ({ ...prev, loading: true, data: null }));

    fetch(API + `/get_timer?uuid=${props.uuid}`).then(async (res) => {
      if (!res.ok) {
        console.error(await res.text());
        return;
      }

      const data = await res.json();
      setState((prev) => ({
        ...prev,
        loading: false,
        data: data[0] as TimerType,
      }));
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
        id: props.uuid,
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

  /**
   *  # Parameters
   *  x -> button type
   *  f -> the time in seconds
   * */
  const updateTimeFunction = (x: "Inc" | "Dec" | "Set", f: number): void => {
    fetch(API + "/change_time", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: props.uuid,
        action: { action: x },
        data: fetchSecondsFromInputs(),
      }),
    });
  };

  let hRef = useRef<HTMLInputElement>(null);
  let mRef = useRef<HTMLInputElement>(null);
  let sRef = useRef<HTMLInputElement>(null);
  let pRef = useRef<HTMLInputElement>(null);

  const fetchSecondsFromInputs = () => {
    const hoursVal = Number(hRef.current?.value || 0) * 3600;
    const minutesVal = Number(mRef.current?.value || 0) * 60;
    const secondsVal = Number(sRef.current?.value || 0);
    const percentageVal = String(pRef.current?.value || 0);

    return [
      String(hoursVal + minutesVal + secondsVal),
      percentageVal.replace("%", ""),
    ];
  };

  const clearValues = () => {
    for (let i of [hRef, mRef, sRef, pRef]) {
      if (i.current) {
        i.current.value = ""
      }
    }
  }


  return (
    <div className={"h-full w-full text-zinc-100 p-2"}>
      <p className={"font-bold text-2xl"}>{state.data?.name}</p>
      <p className={"font-semibold text-zinc-400"}>Uuid {props.uuid}</p>
      <fieldset
        className={"border-1 border-gray-700 rounded-lg p-2 max-w-fit pb-3"}
      >
        <legend className={""}>Paste this into OBS</legend>
        <div
          className={"flex place-items-center bg-gray-700 gap-2 p-2 rounded-lg"}
        >
          <a className={" "} target={"_blank"} href={`/${props.uuid}`}>
            http://
            {window.location.hostname === "localhost"
              ? "localhost:5173"
              : `${HOST}:${PORT}`}
            /{props.uuid}
          </a>
          <ClipboardCopy
            className={"inline cursor-pointer"}
            onClick={() => {
              navigator.clipboard.writeText(
                `http://${window.location.hostname === "localhost" ? "localhost:5173" : `${HOST}:${PORT}`}/${props.uuid}`
              );
            }}
          />
        </div>
      </fieldset>
      <div className={"flex flex-col gap-4 mt-8"}>
        <div>
          <p className={"text-zinc-100 font-semibold text-xl select-none"}>
            Control elements
          </p>
          <div className={"flex gap-2 max-w-fit"}>
            {[
              ["M5", "-5 Min"],
              ["M1", "-1 Min"],
              ["Stop", <Square className={"pointer-events-none"} />],
              ["Play", <Play className={"pointer-events-none"} />],
              ["P1", "+1 Min"],
              ["P5", "+5 Min"],
            ].map(([type, value, ...classnames]) => (
              <button
                key={type}
                id={`control-button-${type}`}
                onClick={buttonOnClick}
                className={
                  // transition-all min-w-10 text-zinc-400 rounded-lg bg-gray-800 p-2 cursor-pointer border border-gray-700
                  `${BUTTON_THEME} py-2
                  ${type === "Stop"
                    ? "hover:text-red-500"
                    : type === "Play"
                      ? "hover:text-green-500"
                      : "hover:text-zinc-100"
                  }
                  ${classnames.join(" ")}
                `
                }
              >
                {value}
              </button>
            ))}
          </div>
        </div>
        <div className={""}>
          <div className={"select-none flex gap-1"}>
            <input
              id={"toggle-timer-white"}
              className={"text-white"}
              type="checkbox"
              checked={state.background_white}
              onChange={() => {
                setState((prev) => ({
                  ...prev,
                  background_white: !prev.background_white,
                }));
              }}
            />
            <label for={"toggle-timer-white"}>Toggle white background</label>
          </div>
          <div className={"flex flex-col gap-2 w-fit"}>
            <fieldset
              className={"border-1 border-gray-700 grow rounded-lg p-2 pb-4"}
            >
              <legend>The current Timer</legend>
              <div className={"flex justify-center"}>
                <iframe
                  src={`/${props.uuid}`}
                  className={`${state.background_white ? "bg-white" : ""}`}
                />
              </div>
            </fieldset>
            <fieldset className={"border-gray-700 border-1 rounded-lg p-2"}>
              <legend title={_T.legend.titles.updateTimes}>
                Increase / Decrease / Set time
              </legend>
              <div className={"flex flex-col gap-2"}>
                <div className={"flex gap-2"}>
                  {[
                    ["Hrs", hRef],
                    ["Mins", mRef],
                    ["Secs", sRef],
                    ["%", pRef],
                  ].map((v, i) => (
                    <input
                      className={
                        "border-1 border-gray-700 max-w-20 min-w-16 grow rounded-lg p-2 py-1"
                      }
                      type={i < 3 ? "number" : "text"}
                      onKeyDown={e => { if (e.key === '-') e.preventDefault(); }}
                      min={0}
                      placeholder={v[0] as string}
                      ref={v[1] as RefObject<HTMLInputElement>}
                    ></input>
                  ))}
                </div>
                <div className={"flex gap-2 select-none "}>
                  <MainButton
                    text="Increase by"
                    onClick={() => updateTimeFunction("Inc", 200)}
                  />
                  <MainButton
                    text="Decrease by"
                    onClick={() => updateTimeFunction("Dec", 200)}
                  />
                  <MainButton
                    text="Set to"
                    onClick={() => updateTimeFunction("Set", 2)}
                  />
                  <MainButton text="clear" onClick={clearValues} />
                </div>
              </div>
            </fieldset>
          </div>
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

  let name = useRef<HTMLInputElement>(null);
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
    <div className={`flex ${props.className} select-none`}>
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
  wsRestart: number;
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

type getTimerRes = {
  payload: string[][] | string;
  type: "tick" | "update";
};

type Actions = {
  type:
  | "UpdateTime"
  | "IncTime"
  | "DecTime"
  | "TogglePause"
  | "UpdateText"
  | "ToggleAnimate"
  | "ToggleLoading"
  | "IncWsRestart"
  | "ResWsRestart";
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

      case "IncWsRestart":
        return {
          ...prev,
          wsRestart: prev.wsRestart + 1,
        };

      case "ResWsRestart":
        return {
          ...prev,
          wsRestart: 0,
        };
      default:
        throw Error("Action is not valid");
    }
  };

  const [state, dispatch] = useReducer(reducer, {
    time: 200, // oh lol
    loading: false,
    text: { upper: "cool text", lower: "cool text" },
    showAnimation: true,
    timerPaused: false,
    wsRestart: 0,
    custom: {
      text: {
        upper: "",
        lower: "",
      },
    },
  });

  const connectWebsocket = () => {
    socket = new WebSocket(BACKEND + "/ws");

    if (!socket) {
      console.error("couldn't open ws connection to the backend. Exiting...");
      return 1;
    }

    socket.onopen = () => {
      console.debug("[Websocket] opened connection...");
      socket!.send(JSON.stringify({ id: getId(), payload: { action: "Reg" } }));
    };

    socket.onmessage = (msg: MessageEvent) => {
      const data: getTimerRes = JSON.parse(msg.data);

      if (data.type === "update") {
        console.debug("Update time due to change");
        const time = data.payload as string;

        dispatch({ type: "UpdateTime", payload: { time: time } });
      }

      if (data.type === "tick") {
        const timerData = Object.fromEntries(data.payload as string[][]);
        const id = getId();

        // eah, check if timer is active in db and "tick" if it is
        if (Object.keys(timerData).includes(id)) {
          dispatch({ type: "UpdateTime", payload: { time: timerData[id] } });
        }
      }
    };

    socket.onclose = () => {
      setTimeout(connectWebsocket, RECONNECT_AFTER * 1000);
    };
  };

  const beforeUnload = () => {
    if (!socket) {
      return;
    }

    socket.send(
      JSON.stringify({
        id: getId(),
        payload: { action: "UnReg" },
      })
    );
    socket.close(15, "Closing timer");
  };

  useEffect(() => {
    // init websocket stuff here
    connectWebsocket();

    fetch(API + `/get_timer?uuid=${getId()}`).then(async (res) => {
      if (!res.ok) {
        console.error(await res.text());
        return;
      }

      const d = await res.json();
      dispatch({ type: "UpdateTime", payload: { time: d[0].timer } });
    });

    addEventListener("beforeunload", beforeUnload);

    return () => {
      removeEventListener("beforeunload", beforeUnload);
    };
  }, []);

  if (state.loading) {
    return <Loading />;
  }

  return (
    <div
      className={`h-screen w-screen text-white flex flex-col items-center ${window.frameElement != null ? "" : "bg-zinc-900"}`}
    >
      <Countdown sec={state.time} showAnimation={state.showAnimation} />
    </div>
  );
};
