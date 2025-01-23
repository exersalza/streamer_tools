import { Icons } from "./Icons";

export function SideBar() {
  return (
    <div className={"h-full bg-zinc-800 w-60 p-2 "}>
      <button className={"inset-ring-2 rounded p-2 inset-ring-indigo-500 text-zinc-100 hover:bg-indigo-500 hover:shadow-indigo-500/50  shadow-lg transition-colors flex"}><span>{Icons.clock}</span>CLICK ME</button>
    </div>
  )
}
