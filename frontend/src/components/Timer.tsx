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
  // I KNOW THERE IS A BETTER WAY
  let name = useRef<HTMLInputElement>();

  let hours = useRef<HTMLInputElement>();
  let minutes = useRef<HTMLInputElement>();
  let seconds = useRef<HTMLInputElement>();

  const blockInvalidChar = (e: KeyboardEvent) => ['e', 'E', '+', '-'].includes(e.key) && e.preventDefault();

  function minMax(e: InputEvent) {

    const target = e.target as HTMLInputElement;
    const targetId = target.id.split("-")[3];
    console.log(targetId)
  
    if (targetId in ["minute", "second"]) {

    }


  }

  return (
    <div className={`${props.hidden ? "hidden" : ""} h-screen w-screen absolute backdrop-blur-xs z-50 grid place-content-center`}>
      <div className={"h-120 w-80 bg-gray-900 rounded-lg flex-col border-1 border-gray-800 p-2"}>
        <p className={"text-zinc-100 text-lg flex flex-col place-items-center "}>Create a timer</p>
        <div className={"flex flex-col"}>
          <label for={"timer-creation-name"} className={"text-zinc-100"}>Timer name</label>
          <input id={"timer-creation-name"} className={"border-1 border-gray-700 rounded text-zinc-100 p-2"} maxlength={50} />

          <label for={"timer-creation-times"} className={"text-zinc-100"}>Initial time</label>
          <div className={"flex place-items-baseline gap-2"}>
            <input id={"timer-creation-times-hour"} className={"border-1 w-20 border-gray-700 rounded text-zinc-100 p-2"} type="number" onKeyDown={blockInvalidChar} onInput={minMax} />

            <p className={"text-zinc-100 text-2xl"}>:</p>
            <input id={"timer-creation-times-minute"} className={"border-1 w-20 border-gray-700 rounded text-zinc-100 p-2"} type="number" onKeyDown={blockInvalidChar} onInput={minMax} />

            <p className={"text-zinc-100 text-2xl"}>:</p>
            <input id={"timer-creation-name-second"} className={"border-1 w-20 border-gray-700 rounded text-zinc-100 p-2"} type="number" onKeyDown={blockInvalidChar} onInput={minMax} />
          </div>
        </div>
      </div>
    </div>
  )
}
