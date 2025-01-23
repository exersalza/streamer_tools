interface HeaderProps  {
  connected: boolean
}

export function Header(props: HeaderProps) {

  return (
    <div className={"h-12 bg-zinc-800 flex place-items-center px-2 justify-between"}>
      <p className={"text-zinc-100 font-semibold"}>&lt;STREAMER_NAME&gt;</p>
      <div className={"flex place-items-center gap-2"}>
        <div className={`w-4 h-4 rounded-full ${props.connected ? "bg-green-500" : "bg-red-500"}`}></div> 
        <p className={"text-zinc-100"}>Connected</p>
      </div>
    </div>
  )
}
