import { render } from 'preact'
import './index.css'
import { Header } from './components/Header'
import { SideBar } from './components/SideBar'
import { useState } from 'preact/hooks'
import { Icons } from './components/Icons'

type States = {
  connected: boolean
}

type TimersType = {
  id: string,
  name: string,
  timer: number,
  increase_times: {
    follow: number | undefined,
    sub_t1: number | undefined,
    sub_t2: number | undefined,
    sub_t3: number | undefined,
    dono_each_n: number | undefined,
    dono_n: number | undefined,
    bits_each_n: number | undefined,
    bits_n: number | undefined,
  }
}


function App() {
  const [states, setStates] = useState<States>({ connected: false });

  return (
    <div className={"bg-gray-950 h-screen flex flex-col"}>
      <Header connected={states.connected} />
      <div className={"h-full"}>
        <SideBar />
      </div>
      <div className={"absolute bottom-1 left-1/2 "}>
        <a href={"https://github.com/exersalza/streamer_tools"} target={"_blank"} className={"text-gray-600 hover:text-gray-500 transition-colors flex place-items-center gap-1 font-semibold select-none"}>Made with <span className={"text-red-500/60"}>{Icons.heart_with_auto_fill}</span> by exersalza</a>
      </div>
    </div>
  )
}



//<div className={"absolute bg-gray-900 w-8 h-8 left-60 top-[2.90rem]"}>
//  <div className={"h-full w-full bg-gray-950 rounded-tl-2xl"}></div>
//</div>
render(<App />, document.getElementById('app')!)
