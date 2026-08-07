import Data from "../data/Data";
import Edit from "../edit/Edit";
import Setting from "../setting/Setting";
import { DisplayHandler, Global } from "../../logic";
import { displayNames, DisplayName } from "./logic";
import Main from "../main/Main";
import { useEffect } from "react";

// タブによって切り替え可能なdisplay群
function Displays(props: {
  displayHandlers: {[key in DisplayName]: DisplayHandler}, 
  global: Global
}) {
  const initDisplay: DisplayName = "main";

  useEffect(() => {
    props.displayHandlers[initDisplay].onOpen();
  }, []);

  return (
    <div className="displays">
      {
        displayNames.map((value) => 
          <div 
            id={"display-" + value} 
            // mainのみ"display display-show"、それ以外は"display"
            className={"display" + (value === initDisplay ? " display-show" : "")} 
            ref={props.displayHandlers[value].content} 
            key={value}
          >
            {
              value === "main"
              ? <Main displayHandler={props.displayHandlers["main"]}/>
              : value === "data"
              ? <Data displayHandler={props.displayHandlers["data"]} global={props.global}/>
              : value === "edit"
              ? <Edit displayHandler={props.displayHandlers["edit"]} global={props.global}/>
              : <Setting displayHandler={props.displayHandlers["setting"]}/>
            }
          </div>
        )
      }
    </div>
  )
}

export default Displays;