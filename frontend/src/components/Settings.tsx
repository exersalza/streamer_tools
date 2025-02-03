interface Props {
  hidden: boolean
}

export function Settings(props: Props) {
  return (
    <div className={`h-screen w-screen absolute z-50 grid place-items-center ${props.hidden ? "hidden" : ""}`}>
      <div className={"bg-gray-800 h-120 w-80 rounded-xl p-2 border-1 border-gray-700 flex flex-col"}>
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
        <div>
          <a href={ "https://id.twitch.tv/oauth2/authorize?response_type=code&client_id=2i56tfmomtm0a3m3m5w83boazvkaks&redirect_uri=http://localhost:22727/api/v1/twitch_auth&scope=channel%3Aread%3Asubscriptions&" }>Connect with Twitch</a>
        </div>
      </div>
    </div>
  )
}
