type HistoryData = {
  event_type: "sub" | "follow" | "bits" | "raid",
  amount: number,
  
}

const History = (props: {history: HistoryData[]}) => {
  return (
    <div className={"flex flex-col gap-2"}>
      {
        props.history.map(v => (
          <div>
            <p>{v.event_type}</p>
          </div>
        ))
      }
    </div>
  )
}
