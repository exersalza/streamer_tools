import { JSX } from "preact/jsx-runtime";
import { stringToHexColor } from "./utils";
import { Fieldset } from "./Customization";

export const History = (props: { history: HistoryData[] }) => {
  const formatAmount = (v: HistoryData): string => {
    const i = v.amount;

    switch (v.event_type) {
      case "gift":
        return i + " Subs";
      case "sub":
        return `T${i}`
      case "cheer":
        return `${i} Bits`
      default:
        return String(i);
    }
  }
  const getFormattedText = (v: HistoryData): (string | JSX.Element)[] => {
    // Split the template by the placeholders
    const template = v.extra;
    const parts = template.split(/(\[user\]|\[amount\]|\[added_time\])/);

    return parts.map((part, i) => {
      switch (part) {
        case '[user]':
          return <span key={`user-${i}`} className={"font-semibold"}>{v.user}</span>;
        case '[amount]':
          return <span key={`amount-${i}`}>{formatAmount(v)}</span>;
        case '[added_time]':
          return <span key={`time-${i}`}><span className={"font-semibold"}>{v.time_added}</span> seconds</span>;
        default:
          return part;
      }
    });
  };

  return (

    <Fieldset className={"border-1 border-gray-700 rounded-lg p-2 overflow-auto h-70 "} legend="Event History">
      <div className={"flex flex-col gap-2"}>
        {
          // we reverse here as we dont want to shift all elements in the array when we add a new event
          props.history.map((v, i) => (
            <div className={"border-1 border-gray-700 p-2 rounded-lg"} key={i}>
              <div className={"flex gap-2"}>
                <p>{new Date(v.timestamp * 1000).toLocaleTimeString()}</p>
                <p
                  className={""}
                  style={{ color: `${stringToHexColor(v.event_type)}` }}
                >
                  {v.event_type}
                </p>
                <p>{getFormattedText(v)}</p>
              </div>
            </div>
          ))
        }
      </div>
    </Fieldset>
  );
};
