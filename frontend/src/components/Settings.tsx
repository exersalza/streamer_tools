import { useEffect, useRef, useState } from "preact/hooks"
import { API } from "./utils";
import { Dropdown } from "./Dropdown";

interface Props {
  hidden: boolean
}

interface User {
  id: string,
  login: string,
  display_name: string,
  profile_pic: string,
  broadcaster_type: string
}

export function Settings(props: Props) {
  const [twitchConnected, setTwitchConnected] = useState(false);
  const [user, setUser] = useState<User>();

  useEffect(() => {
    fetch(API + "/is_connected_to_twitch").then(async (res) => {
      if (!res.ok) {
        return
      }

      let f = await res.text();
      setTwitchConnected(f === "true");
    });

    fetch(API + "/get_user").then(async (res) => {
      if (!res.ok) {
        return
      }

      let f = await res.json();
      setUser(f);
    });
  }, [])

  const updateUser = () => {
    fetch(API + "/update_user", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        username: user?.login ?? ""
      })
    }).then(async (res) => {
      if (!res.ok) {
        return
      }
    })
  }

  return (
    <div className={`h-screen w-screen absolute z-50 grid place-items-center ${props.hidden ? "hidden" : ""}`}>
      <div className={"bg-gray-800 h-120 w-80 rounded-xl p-2 border-1 border-gray-700 flex flex-col gap-4"}>
        <p className={"text-zinc-100 font-semibold"}>Settings</p>
        <div>
          <fieldset className={"border-2 border-gray-700 p-2 rounded"}>
            <legend className={"text-zinc-400"}>User Related stuff</legend>
            <form className={"flex flex-col"}>
              <label for={"twitch-user"} className={"text-zinc-100"} title={"This is used to get user specific stuff"} >User login</label>
              <input onInput={(e) => { setUser((prev) => ({ ...prev, login: (e.target as HTMLInputElement).value })) }} id={"twitch-user"} value={user?.login} title={"This is used to get user specific stuff"} className={"border-1 border-gray-500 rounded text-zinc-100 px-2"}></input>
              <button className={"text-zinc-100 cursor-pointer"} type={"submit"} onClick={(e) => { e.preventDefault(); updateUser() }}>Save</button>
            </form>
          </fieldset>
        </div>
        <div className={"flex flex-col gap-1"}>
          <a className={"bg-purple-500 text-zinc-100 font-semibold p-2 rounded"} href={"https://id.twitch.tv/oauth2/authorize?response_type=code&client_id=2i56tfmomtm0a3m3m5w83boazvkaks&force_verify=true&redirect_uri=http://localhost:22727/api/v1/twitch_auth&scope=channel.read.subscriptions&"}>Connect with Twitch</a>
          <p className={`${!twitchConnected ? "text-red-400" : "text-green-400"}`}>{twitchConnected ? "Connected" : "Not Connected"}</p>
        </div>
        <div>
          <button className={"cursor-pointer text-zinc-100 bg-gray-700 rounded p-2"}>Close</button>
        </div>
      </div>
    </div>
  )
}
