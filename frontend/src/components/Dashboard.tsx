import { useEffect, useState } from "preact/hooks"
import { TimerType } from "../main"
import { API } from "./utils";
import { Timer, TimerButton } from "./Timer";
import { Icons } from "./Icons";

type States = {
  timer: TimerType[]
}


export function Dashboard() {
  const [state, setState] = useState<States>({ timer: [] });

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
    <div className={"w-full h-full p-2 flex flex-col gap-8"}>
      <p className={"text-zinc-100 text-2xl"}>Dashboard</p>
      <div className={"flex flex-col"}>
        <p className={"text-zinc-200 text-lg"}>Buttons (idk what to put here yet)</p>
        <div className={"flex gap-2"}>
          <button
            className={`rounded transition-all h-8 bg-gray-800 hover:bg-gray-700 min-w-40 cursor-pointer`}>
            <p className={"text-zinc-100 flex gap-2 px-1 place-items-center"}>{Icons.circle_add} Create timer</p>
          </button>

        </div>
      </div>

      <div className={"flex flex-col gap-2"}>
        <p className={"text-zinc-200 text-lg"}>All timers</p>
        <div className={"flex gap-2"}>
          {state.timer.map((timer) => {
            return <TimerButton data={timer} active={""} />
          })}
        </div>
      </div>
    </div>
  )
}
