// import { useRef } from "react";
import Data from "../data/Data";
import Edit from "../edit/Edit";
import { DisplayHandler, DisplayName, SpecialFunctions } from "../../logic";

function Displays(props: {displayHandlers: {[key in DisplayName]: DisplayHandler}, specialFunctions: SpecialFunctions}) {

  return (
    <div className="displays">
      <div id="display-main" className="display display-show" ref={props.displayHandlers["main"].content}>
        メイン
      </div>
      <div id="display-data" className="display" ref={props.displayHandlers["data"].content}>
        <Data specialFunctions={props.specialFunctions}/>
      </div>
      <div id="display-edit" className="display" ref={props.displayHandlers["edit"].content}>
        <Edit displayHandler={props.displayHandlers["edit"]} specialFunctions={props.specialFunctions}/>
      </div>
      <div id="display-set" className="display" ref={props.displayHandlers["set"].content}>
        設定
      </div>
    </div>
  )
}

export default Displays;