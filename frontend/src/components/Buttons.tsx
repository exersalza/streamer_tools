
export const BUTTON_THEME = "border-1 border-gray-700 p-2 py-1 rounded-lg grow transition-all hover:cursor-pointer hover:bg-gray-700";

export const MainButton = (props: { text: string, onClick: () => void }) => {
  return (
    <button className={BUTTON_THEME} onClick={props.onClick}>{props.text}</button>
  )
}
