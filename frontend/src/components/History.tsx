import { stringToHexColor } from "./utils"

export const History = (props: { history: HistoryData[] }) => {
  return (
    <fieldset
      className={"border-1 border-gray-700 rounded-lg p-2 pb-3"}
    >
      <legend>Event History</legend>
      <div className={"flex flex-col gap-2"}>
        {
          // we reverse here as we dont want to shift all elements in the array when we add a new event
          props.history.reverse().map(v => (
            <div className={"border-1 border-gray-700 p-2"}>
              <div className={"flex gap-2"}>
                <p className={""} style={{ color: `${stringToHexColor(v.event_type)}`}}>{v.event_type}</p>
                <p>{v.user}</p>
                <p>{new Date(v.timestamp * 1000).toLocaleString()}</p>
              </div>
            </div>
          ))
        }
      </div>
    </fieldset>
  )
}
