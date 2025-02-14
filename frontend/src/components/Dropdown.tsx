import { useEffect, useRef, useState } from "preact/hooks";
import { Icons } from "./Icons";

interface Props {
  values: string[],
  disabled?: boolean,
  placeholder?: string,
  callback: (value: string) => void;
}

type DropdownStates = {
  open: boolean,
  values: string[],
  searchReg: RegExp,
  lastSelectedValue: string
}

export function Dropdown(props: Props) {
  const [states, setStates] = useState<DropdownStates>({
    open: false,
    values: props.values,
    searchReg: /.*/g,
    lastSelectedValue: ""
  })

  let itemsRef = useRef<HTMLDivElement>(null);
  let bodyRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setStates((prev) => ({ ...prev, values: props.values }));
  }, [props.values])


  const inputHandler = (e: InputEvent) => {
    const value = (e.target as HTMLInputElement).value;
    const reg = new RegExp(`.*(${value}).*`, "gi");

    setStates((prev) => ({
      ...prev,
      values: props.values.filter((i) => {
        reg.lastIndex = 0; return reg.test(i)
      }),
      lastSelectedValue: value
    }))
  }

  // make the dropdown close when you click outside of it
  const outsideClickHandler = (e: MouseEvent) => {
    const target = e.target as Node;

    if (!!bodyRef.current && !bodyRef.current.contains(target)) {
      setStates((prev) => ({ ...prev, open: false }));
    }
  }

  function cb(value: string) {
    props.callback(value);
    setStates((prev) => ({ ...prev, open: false, lastSelectedValue: value }));
  }

  useEffect(() => {
    document.addEventListener("click", outsideClickHandler);

    return () => {
      document.removeEventListener("click", outsideClickHandler);
    }
  }, [])


  return (
    <div className={`w-46 max-h-60 relative z-100 bg-gray-100 ${states.open ? "rounded-t-lg" : "rounded-lg"} select-none transition-all dropdown-parent`} ref={bodyRef}>
      <div className={"flex place-items-center "} onClick={() => {
        setStates((prev) => ({ ...prev, open: true }))
      }}>
        <input
          onInput={inputHandler}
          className={"px-1 py-1 outline-none w-8/9"}
          placeholder={props.placeholder ?? "Search..."}
          value={states.lastSelectedValue}
        />
        <p onClick={(e) => {
          e.stopPropagation();
          setStates((prev) => ({ ...prev, open: !prev.open }))
        }}>

          {Icons.chevron_down}
        </p>
      </div>
      <div className={`px-1 max-h-49 w-full absolute flex flex-col transition-all overflow-auto good-scrollbar rounded-b-lg scrollbar scrollbar-thumb-zinc-400 bg-gray-100 duration-100`}
        ref={itemsRef}
        style={{
          height: `calc(${states.open ? Math.min(props.values.length, 9) : 0} * 1.5rem)`,
          top: '100%',
          visibility: states.open ? 'visible' : 'hidden',
          opacity: states.open ? 1 : 0,
        }}>
        {states.values.map((value) => {
          return <p
            onClick={() => cb(value)}
            className={"cursor-pointer hover:bg-gray-200 rounded"}>
            {value}
          </p>
        })}
      </div>
    </div>
  )
}
