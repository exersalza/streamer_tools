import { useRef, useState } from "preact/hooks";
import { TimerType } from "../main"
import { Icons } from "./Icons";
import { ChangeEvent } from "preact/compat";
import { API } from "./utils";
import { HexColorPicker, HexColorInput } from "powerful-color-picker";

type States = {}

interface TimerButtonProps {
  data: TimerType
  active: string
}


export function TimerButton(props: TimerButtonProps) {
  const [state, setState] = useState<States>({});

  return (
    <button
      key={props.data.id}
      className={`cursor-pointer rounded transition-all h-8 ${props.active === props.data.id ? "bg-gray-700 hover:bg-gray-600" : "bg-gray-800 hover:bg-gray-700"} min-w-40`}>
      <p className={"text-zinc-100 flex gap-2 px-1"}>{Icons.clock} {props.data.name}</p>
    </button>
  )
}


interface TimerProps {
  name: string,
  uuid: string,
}

export function Timer(props: TimerProps) {
  return (
    <div className={"h-full w-full"}>
      <p>Timer {props.name}</p>
    </div>
  )
}


interface TimerOverlayProps {
  hidden: boolean,
  hideWindow: () => void,
  update: (prev: number) => void
}

export function CreateTimerOverlay(props: TimerOverlayProps) {
  const [values, setValues] = useState(
    {
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
    },
  );

  const [showColorPick, setShowColorPick] = useState(false);
  const [color, setColor] = useState("#aabbcc");

  const blockInvalidChar = (e: KeyboardEvent) => ['e', 'E', '+', '-'].includes(e.key) && e.preventDefault();
  const blockInvalidHex = (e: KeyboardEvent) => !/[\#a-fA-F0-9]/.test(e.key) && e.preventDefault();

  const value_thing = {
    // change titles
    "follow": ["Follow", ""],
    "sub_t1": ["Tier 1 Sub", ""],
    "sub_t2": ["Tier 2 Sub", ""],
    "sub_t3": ["Tier 3 Sub", ""],
    "bits_each_n": ["(Bits) X amount spent", "border-yellow-700"],
    "bits_n": ["(Bits) Increase time by", "border-yellow-700"],
    "dono_each_n": ["(Dono) X amount spent", "border-green-700"],
    "dono_n": ["(Dono) Increase time by", "border-green-700"],
  }

  let name = useRef<HTMLInputElement>();

  function changeColor(e: InputEvent) {
    let intar = (e.target as HTMLInputElement).value ?? "";

    if (!/^#?[a-fA-F0-9]{0,6}$/gm.test(intar)) return

    setColor(intar);
  }

  function minMax(e: InputEvent) {
    const target = e.target as HTMLInputElement;
    const targetId = target.id.split("-")[3];
    setValues((prev) => ({ ...prev, [targetId]: target.valueAsNumber }));

    if (targetId === "hour") {
      if (target.value.length > 6) {
        setValues((prev) => ({ ...prev, hour: target.value.slice(0, 6) }))
      }
    }

    if (targetId === "minute" || targetId === "second") {
      if (target.value.length > 2) {
        setValues((prev) => ({ ...prev, [targetId]: target.value.slice(0, 2) }))
      }
    }

    if (target.valueAsNumber < 0) {
      setValues((prev) => ({ ...prev, [targetId]: 0 }))
    }

    if (targetId === "minute" || targetId === "second") {
      if (target.valueAsNumber >= 59) {
        setValues((prev) => ({ ...prev, [targetId]: 59 }))
      }
    };
  }

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
        timer: (values.hour * 60 * 60) + (values.minute * 60) + values.second,
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
        }
      })
    }).then(async (res) => {
      if (!res.ok) console.log(await res.text());
      props.update(Math.random());
      props.hideWindow();
    });
  }

  return (
    <div className={`${props.hidden ? "hidden" : ""} transition-all h-screen w-screen absolute backdrop-blur-xs z-50 grid place-content-center`}>
      <div className={"h-190 w-80 bg-gray-900 rounded-lg flex-col border-1 border-gray-800 p-2"}>
        <p className={"text-zinc-100 text-lg flex flex-col place-items-center "}>Create a timer</p>
        <div className={"flex flex-col gap-2"}>
          <label for={"timer-creation-name"} className={"text-zinc-100"}>Timer name</label>
          <input id={"timer-creation-name"} placeholder={"what-a-timer"} ref={name} className={"border-1 border-gray-700 rounded text-zinc-100 p-2"} maxlength={50} />

          <label for={"timer-creation-times"} className={"text-zinc-100"}>Initial time</label>
          <div className={"flex place-items-baseline gap-2"}>
            <input id={"timer-creation-times-hour"} placeholder={"Hrs..."} className={"border-1 w-25 border-gray-700 rounded text-zinc-100 p-2"} type="number" onKeyDown={blockInvalidChar} onInput={minMax} value={values.hour} />

            <p className={"text-zinc-100 text-2xl"}>:</p>
            <input id={"timer-creation-times-minute"} placeholder={"Min..."} className={"border-1 w-20 border-gray-700 rounded text-zinc-100 p-2"} type="number" onKeyDown={blockInvalidChar} onInput={minMax} value={values.minute} />

            <p className={"text-zinc-100 text-2xl"}>:</p>
            <input id={"timer-creation-times-second"} placeholder={"Sec..."} className={"border-1 w-20 border-gray-700 rounded text-zinc-100 p-2"} type="number" onKeyDown={blockInvalidChar} onInput={minMax} value={values.second} />
          </div>
          <div className={"flex flex-col gap-2"}>
            {
              Object.keys(value_thing).map((key) => {
                return (
                  <div className={"flex place-items-center gap-2"}>
                    <input id={`timer-creation-increases-${key}`} value={values[key]} className={`border-1 w-20 border-gray-700 rounded text-zinc-100 p-2 ${value_thing[key][1]}`} type="number" onKeyDown={blockInvalidChar} onInput={minMax} />
                    <p className={`text-zinc-100 }`}>{value_thing[key][0]}</p>
                  </div>
                )
              })
            }
          </div>
        </div>
        <div>
          <div className={"flex flex-col gap-2"}>
            <div className={"flex gap-2"} title={"Is this the main subathon timer (does subs, and bits increase this timer or not)"}>
              <input type="checkbox" id={"timer-creation-main"} checked={values.main} onChange={() => { setValues((prev) => ({ ...prev, main: !prev.main })) }} />
              <label for={"timer-creation-main"} className={"text-zinc-100 select-none"}>Mark as Main timer</label>
            </div>
            <div className={"flex gap-2"} title={"Should the timer be set active upon creation"}>
              <input type="checkbox" id={"timer-creation-is_active"} checked={values.is_active} onChange={() => { setValues((prev) => ({ ...prev, is_active: !prev.is_active })) }} />
              <label for={"timer-creation-is_active"} className={"text-zinc-100 select-none"}>Is timer active</label>
            </div>
            <div className={`absolute -translate-y-50 ${showColorPick ? "" : "hidden"} p-1 rounded-lg bg-gray-700 flex flex-col gap-2`}>
              <HexColorPicker color={color} onChange={setColor} />
              <input className={"px-1 ring-1 ring-zinc-500 rounded text-zinc-100"} value={color} onInput={changeColor} />
              <button className={"text-zinc-100 cursor-pointer hover:bg-gray-600 rounded"} onClick={() => {setShowColorPick(false)}}>Save</button>
            </div>
            <button className={"text-zinc-100 cursor-pointer"} onClick={() => { setShowColorPick((prev) => !prev) }}>Spawn color picker</button>
          </div>
        </div>
        <div className={"flex w-full gap-2 mt-3"}>
          <button
            onClick={createTimer}
            className={`cursor-pointer text-zinc-100 rounded transition-all h-8 w-full bg-gray-700 hover:bg-gray-600`}>
            Create
          </button>
          <button
            onClick={props.hideWindow}
            className={`cursor-pointer text-zinc-100 rounded transition-all h-8 w-full bg-gray-700 hover:bg-gray-600`}>
            Abort
          </button>
        </div>
      </div>
    </div>
  )
}
