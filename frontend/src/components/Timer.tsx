import { useRef, useState } from "preact/hooks";
import { TimerType } from "../main"
import { Icons } from "./Icons";
import { ChangeEvent } from "preact/compat";

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
  uuid: string
}

export function Timer(props: TimerProps) {
  return (
    <div className={"h-full w-full"}>
      <p>Timer {props.name}</p>
    </div>
  )
}


interface TimerOverlayProps {
  hidden: boolean
}

export function CreateTimerOverlay(props: TimerOverlayProps) {
  const [values, setValues] = useState({ hour: "", minute: "", second: "", main: false });

  const blockInvalidChar = (e: KeyboardEvent) => ['e', 'E', '+', '-'].includes(e.key) && e.preventDefault();

  const value_thing = {
    // change titles
    "follow": [ "Follow", "" ],
    "sub_t1": [ "Tier 1 Sub", "" ],
    "sub_t2": [ "Tier 2 Sub", "" ],
    "sub_t3": [ "Tier 3 Sub", "" ],
    "bits_each_n": [ "(Bits) X amount spent", "border-yellow-700" ],
    "bits_n": [ "(Bits) Increase time by", "border-yellow-700" ],
    "dono_each_n": [ "(Dono) X amount spent", "border-green-700" ],
    "dono_n": [ "(Dono) Increase time by", "border-green-700" ],
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

    if (targetId === "hour") return;

    let v = target.valueAsNumber;

    console.log(targetId)
    if (v >= 59) {
      setValues((prev) => ({ ...prev, [targetId]: 59 }))
    }

    if (v < 0) {
      setValues((prev) => ({ ...prev, [targetId]: 0 }))
    }
  }

  return (
    <div className={`${props.hidden ? "hidden" : ""} h-screen w-screen absolute backdrop-blur-xs z-50 grid place-content-center`}>
      <div className={"h-170 w-80 bg-gray-900 rounded-lg flex-col border-1 border-gray-800 p-2"}>
        <p className={"text-zinc-100 text-lg flex flex-col place-items-center "}>Create a timer</p>
        <div className={"flex flex-col gap-2"}>
          <label for={"timer-creation-name"} className={"text-zinc-100"}>Timer name</label>
          <input id={"timer-creation-name"} placeholder={"what-a-timer"} className={"border-1 border-gray-700 rounded text-zinc-100 p-2"} maxlength={50} />

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
                    <input id={`timer-creation-${key}`} className={`border-1 w-20 border-gray-700 rounded text-zinc-100 p-2 ${value_thing[key][1]}`} type="number" onKeyDown={blockInvalidChar} />
                    <p className={`text-zinc-100 }`}>{value_thing[key][0]}</p>
                  </div>
                )
              })
            }
          </div>
        </div>
        <div className={"mt-2 flex gap-2"}  title={"something"}>
          <input type="checkbox" id={"timer-creation-main"} checked={values.main} onChange={() => { setValues((prev) => ({ ...prev, main: !prev.main })) }} />
          <label for={"timer-creation-main"} className={"text-zinc-100 select-none"}>Mark as Main timer</label>
        </div>
        <div className={"flex w-full gap-2 mt-3"}>
          <button
            onClick={() => { }}
            className={`cursor-pointer text-zinc-100 rounded transition-all h-8 w-full bg-gray-700 hover:bg-gray-600`}>
            Create
          </button>
          <button
            onClick={() => { }}
            className={`cursor-pointer text-zinc-100 rounded transition-all h-8 w-full bg-gray-700 hover:bg-gray-600`}>
            Abort
          </button>
        </div>
      </div>
    </div>
  )
}
