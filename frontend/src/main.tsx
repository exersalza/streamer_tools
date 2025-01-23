import { render } from 'preact'
import './index.css'
import { Header } from './components/Header'
import { SideBar } from './components/SideBar'
import { useState } from 'preact/hooks'

type States = {
  connected: boolean
}

function App() {
  const [states, setStates] = useState<States>({ connected: false });
  return (
    <div className={"bg-zinc-900 h-screen flex flex-col"}>
      <Header connected={states.connected} />
      <div className={"h-full"}>
        <SideBar />
      </div>
      <div className={"absolute bg-zinc-800 w-8 h-8 left-60 top-[2.90rem]"}>
        <div className={"h-full w-full bg-zinc-900 rounded-tl-2xl"}></div>
      </div>
    </div>
  )
}

render(<App />, document.getElementById('app')!)
