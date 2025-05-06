export const BUTTON_THEME =
  "select-none border-1 border-gray-700 p-2 py-1 rounded-lg grow transition-all hover:cursor-pointer hover:bg-gray-700 "; // this space here is important

export const MainButton = (props: {
  text: string;
  disabled?: boolean;
  className?: string;
  onClick: (e: MouseEvent) => void;
  type?: "submit" | "reset" | "button";
}) => {
  return (
    <button
      type={props.type || "button"}
      disabled={props.disabled}
      className={BUTTON_THEME + props.className}
      onClick={props.onClick}
    >
      {props.text}
    </button>
  );
};
