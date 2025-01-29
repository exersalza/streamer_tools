import { useEffect, useState } from "preact/hooks";
import { Icons } from "./Icons";
import { TimerType } from "../main";
import { API } from "./utils";
import { TimerButton } from "./Timer";

type States = {
  timer: TimerType[],
  current_view_timer: string
}

interface Props {
  active: string
  setActiveTab: any
}

export function SideBar(props: Props) {
  const [state, setState] = useState<States>({ timer: [], current_view_timer: "" });

  useEffect(() => {
    fetch(API + "/get_all_timers").then(async (res) => {
      if (!res.ok) {
        console.error(res.text());
        return;
      }

      const d = await res.json();
      setState((prev) => ({ ...prev, timer: d }));
    })
  }, [])

  return (
    <div className={"h-full bg-gray-900 w-60 p-2 flex flex-col gap-2 border-r-1 border-gray-800"}>
      <div className={"flex flex-col h-1/3"}>
        <p className={"text-zinc-100 text-xl font-semibold"}>Utils</p>
        <div className={"flex flex-col"}>

          <button
            className={`cursor-pointer rounded transition-all h-8 ${props.active === "dashboard" ? "bg-gray-700 hover:bg-gray-600" : "bg-gray-800 hover:bg-gray-700"}`}>
            <p className={"text-zinc-100 flex gap-2 place-items-center px-1"}>{Icons.dashboard} Dashboard</p>
          </button>
        </div>
      </div>

      <div className={"flex flex-col gap-2"}>
        <p className={"text-zinc-100 text-xl font-semibold"}>Timer</p>
        {state?.timer.map((timer) => {
          return <TimerButton data={timer} active={state.current_view_timer} />
        })}
      </div>
    </div>
  )
}
