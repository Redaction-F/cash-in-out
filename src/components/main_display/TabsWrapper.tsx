import { DisplayHandler, DisplayName } from "../../logic";

function TabsWrapper(props: {displayHandlers: {[key in DisplayName]: DisplayHandler}, changeDisplay: (tab_name: DisplayName) => Promise<void>}) {
  return (
    <div className="tabs-wrapper">
      <div className="tab">
        <input id="tab-main" className="tab-button" type="radio" name="tab" value="main" onChange={props.changeDisplay.bind(window, "main")} ref={props.displayHandlers["main"].tab} defaultChecked />
        <label className="tab-label" htmlFor="tab-main">
          メイン
        </label>
      </div>
      <div className="tab">
        <input id="tab-data" className="tab-button" type="radio" name="tab" value="data" onChange={props.changeDisplay.bind(window, "data")} ref={props.displayHandlers["data"].tab} />
        <label className="tab-label" htmlFor="tab-data">
          データ
        </label>
      </div>
      <div className="tab">
        <input id="tab-edit" className="tab-button" type="radio" name="tab" value="edit" onChange={props.changeDisplay.bind(window, "edit")} ref={props.displayHandlers["edit"].tab} />
        <label className="tab-label" htmlFor="tab-edit">
          編集
        </label>
      </div>
      <div className="tab">
        <input id="tab-set" className="tab-button" type="radio" name="tab" value="set" onChange={props.changeDisplay.bind(window, "set")} ref={props.displayHandlers["set"].tab} />
        <label className="tab-label" htmlFor="tab-set">
          設定
        </label>
      </div>
    </div>
  )
}

export default TabsWrapper;