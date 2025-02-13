import { RefObject, useRef } from "react";
import Data from "../data/Data";
import Edit from "../edit/Edit";

function Displays(props: {tab_names: string[], setDisplayContents: (display_contents_arg: {[key: string]: RefObject<HTMLDivElement>}) => void}) {
  const display_contents: {[key: string]: RefObject<HTMLDivElement>} = {};
  props.tab_names.forEach((value: string) => {
    display_contents[value] = useRef<HTMLDivElement>(null);
  });
  props.setDisplayContents(display_contents);

  return (
    <div className="displays">
      <div id="display-main" className="display display-show" ref={display_contents["main"]}>
        メイン
      </div>
      <div id="display-data" className="display" ref={display_contents["data"]}>
        <Data />
      </div>
      <div id="display-edit" className="display" ref={display_contents["edit"]}>
        <Edit />
      </div>
      <div id="display-set" className="display" ref={display_contents["set"]}>
        設定
      </div>
    </div>
  )
}

export default Displays;