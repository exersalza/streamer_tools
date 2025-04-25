import { useEffect, useErrorBoundary, useState } from "preact/hooks"
import { API, HOST } from "./utils";
import Cookies from 'js-cookie'
import { Icon, Settings } from "lucide-preact";
import { Icons } from "./Icons";

type HeaderStates = {
  username: string
}

export function Header(props: {connected: boolean, toggleSettings: () => void}) {
  const [states, setStates] = useState<HeaderStates>({
    username: ""
  });

  useEffect(() => {
    // we gotta put an option in to remove website data
    const username = Cookies.get("username");
    if (username !== undefined) {
      setStates((prev) => ({...prev, username: username ?? ""}))
    };


    // get username from backend and put it inside the state
    fetch(API + "/get_twitch_username").then(async (res) => {
      if (!res.ok) {
        console.error(await res.text());
        return
      }

      const username = await res.text();
      Cookies.set("username", username);
      setStates((prev) => ({ ...prev, username: username }))

    }).catch((e) => { console.error(e) })
  }, [])

  return (
    <div className={"h-12 bg-gray-800 flex place-items-center px-2 justify-between border-b-1 border-gray-700"}>
      <p className={"text-zinc-100 font-semibold"}>{states.username}</p>
      <Settings onClick={props.toggleSettings} className={"text-gray-100"} />
      <div className={"flex place-items-center gap-2"}>
        <div className={`w-4 h-4 rounded-full ${props.connected ? "bg-green-500" : "bg-red-500"}`}></div>
        <p className={"text-zinc-100"}>{props.connected ? "" : "Not"} Connected</p>
      </div>
    </div>
  )
}
