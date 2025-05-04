import { render } from "preact";
import "./index.css";
import { Header } from "./components/Header";
import { SideBar } from "./components/SideBar";
import { useEffect, useState } from "preact/hooks";
import { Icons } from "./components/Icons";
import { Dashboard } from "./components/Dashboard";
import { CreateTimerOverlay, Timer, TimerWidget } from "./components/Timer";
import { Settings } from "./components/Settings";
import { API, BACKEND } from "./components/utils";

type States = {
  connected: boolean;
  showSettings: boolean;
  showCreateTimerOverlay: boolean;
};

export type TimerType = {
  id: string;
  name: string;
  timer: number;
  color: string;
  main: boolean;
  increase_times: {
    follow: number | undefined;
    sub_t1: number | undefined;
    sub_t2: number | undefined;
    sub_t3: number | undefined;
    dono_each_n: number | undefined;
    dono_n: number | undefined;
    bits_each_n: number | undefined;
    bits_n: number | undefined;
  };
};

function App() {
  const [states, setStates] = useState<States>({
    connected: false,
    showSettings: false,
    showCreateTimerOverlay: false,
  });
  const [activeTab, setActiveTab] = useState("dashboard");
  const [update, setUpdate] = useState(0);
  const [timerIds, setTimerIds] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {}, [update]);

  useEffect(() => {
    fetch(API + "/get_timer_names").then(async (res) => {
      if (!res.ok) {
        console.error(await res.text());
        return;
      }

      let d = await res.json();
      setTimerIds(d);
      setLoading(false);
    });
  }, []);

  const setActiveTimer = (id: string) => setActiveTab(id);

  if (loading) {
    return <div>{Icons.beer_spin}</div>;
  }

  return (
    <div>
      {timerIds.includes(location.pathname.replace("/", "")) ? (
        <TimerWidget />
      ) : (
        <div className={"bg-gray-950 h-screen flex flex-col"}>
          <Header
            connected={states.connected}
            toggleSettings={() => {
              setStates((prev) => ({
                ...prev,
                showSettings: !prev.showSettings,
              }));
            }}
          />
          <Settings
            hidden={!states.showSettings}
            setHidden={() =>
              setStates((prev) => ({ ...prev, showSettings: false }))
            }
          />
          <CreateTimerOverlay
            hidden={!states.showCreateTimerOverlay}
            hideWindow={() => {
              setStates((prev) => ({ ...prev, showCreateTimerOverlay: false }));
            }}
            update={setUpdate}
          />

          <div className={"h-full flex"}>
            <SideBar
              active={activeTab}
              setActiveTab={setActiveTimer}
              update={update}
            />
            <div className={"w-full"}>
              {activeTab === "dashboard" ? (
                <Dashboard
                  update={update}
                  openTimerOverlay={() => {
                    setStates((prev) => ({
                      ...prev,
                      showCreateTimerOverlay: !prev.showCreateTimerOverlay,
                    }));
                  }}
                  setActiveTimer={setActiveTimer}
                />
              ) : (
                <Timer uuid={activeTab} />
              )}
            </div>
          </div>
          <div
            className={
              "absolute flex h-screen w-screen justify-center items-end pointer-events-none"
            }
          >
            <a
              href={"https://github.com/exersalza/streamer_tools"}
              target={"_blank"}
              className={
                "pointer-events-auto text-gray-600 hover:text-gray-500 transition-colors text-sm flex place-items-center gap-1 font-semibold select-none"
              }
            >
              Made with{" "}
              <span
                title={"Its beer, or is it?"}
                onClick={() => {
                  window.location.assign(BACKEND + "/fish");
                }}
              >
                {Icons.beer}
              </span>{" "}
              by exersalza
            </a>
          </div>
        </div>
      )}
    </div>
  );
}

//<div className={"absolute bg-gray-900 w-8 h-8 left-60 top-[2.90rem]"}>
//  <div className={"h-full w-full bg-gray-950 rounded-tl-2xl"}></div>
//</div>
render(<App />, document.getElementById("app")!);
