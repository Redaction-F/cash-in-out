import Data from "../data/Data";
import Edit from "../edit/Edit";
import { DisplayHandler, SpecialFunctions } from "../../logic";
import { displayName, DisplayName } from "./logic";

// タブによって切り替え可能なdisplay群
function Displays(props: {displayHandlers: {[key in DisplayName]: DisplayHandler}, specialFunctions: SpecialFunctions}) {
  return (
    <div className="displays">
      {
        displayName.map((value) => 
          <div 
            id={"display-" + value} 
            // mainのみ"display display-show"、それ以外は"display"
            className={"display" + (value === "main" ? " display-show" : "")} 
            ref={props.displayHandlers[value].content} 
            key={value}
          >
            {
              value === "main"
              ? "メイン"
              : value === "data"
              ? <Data displayHandler={props.displayHandlers["data"]} specialFunctions={props.specialFunctions}/>
              : value === "edit"
              ? <Edit displayHandler={props.displayHandlers["edit"]} specialFunctions={props.specialFunctions}/>
              : "設定"
            }
          </div>
        )
      }
    </div>
  )
}

export default Displays;