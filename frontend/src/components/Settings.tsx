import { useEffect, useState } from "preact/hooks"
import { API } from "./utils";

interface Props {
  hidden: boolean
}

export function Settings(props: Props) {
  const [twitchConnected, setTwitchConnected] = useState(false);


  useEffect(() => {
    fetch(API + "/is_connected_to_twitch").then(async (res) => {
      if (!res.ok) {
        return
      }

      let f = await res.text();
      console.log(f);
      setTwitchConnected(f === "true");
    })
   }, [])

  return (
    <div className={`h-screen w-screen absolute z-50 grid place-items-center ${props.hidden ? "hidden" : ""}`}>
      <div className={"bg-gray-800 h-120 w-80 rounded-xl p-2 border-1 border-gray-700 flex flex-col gap-4"}>
        <p className={"text-zinc-100 font-semibold"}>Settings</p>
        <div>
            <fieldset className={"border-2 border-gray-700 p-2 rounded"}>
            <legend className={"text-zinc-400"}>Something</legend>
            <form className={"flex flex-col"}>
              <label for={"twitch-token"} className={"text-zinc-100"}>Token</label>
              <input type="password" id={"twitch-token"} className={"border-1 border-gray-500 rounded text-zinc-100 px-2"}></input>
              <a href={"#"}>Get your token here</a>
            </form>
          </fieldset>
        </div>
        <div className={"flex flex-col gap-1"}>
          <a className={"bg-purple-500 text-zinc-100 font-semibold p-2 rounded"} href={ "https://id.twitch.tv/oauth2/authorize?response_type=code&client_id=2i56tfmomtm0a3m3m5w83boazvkaks&force_verify=true&redirect_uri=http://localhost:22727/api/v1/twitch_auth&scope=channel%3Aread%3Asubscriptions&" }>Connect with Twitch</a>
          <p className={`${!twitchConnected ? "text-red-400" : "text-green-400"}`}>{twitchConnected ? "Connected" : "Not Connected"}</p>
        </div>
      </div>
    </div>
  )
}
