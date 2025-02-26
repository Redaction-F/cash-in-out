import { isDisplayName, DisplayHandler, DisplayName, displayName } from "../../logic";

// display群を切り替えるためのtab群
function TabsWrapper(props: {displayHandlers: {[key in DisplayName]: DisplayHandler}, changeDisplay: (tab_name: DisplayName) => Promise<boolean>}) {
  // tabのonChangeからdisplay切り替えをするためのラッパー
  function changeDisplayFromTab(event: React.ChangeEvent<HTMLInputElement>) {
    let value: string = event.target.value;
    if (isDisplayName(value)) {
      props.changeDisplay(value);
    }
  }

  return (
    <div className="tabs-wrapper">
      {
        displayName.map((value) => 
          <div className="tab" key={value}>
            <input 
              // label関連付け用
              id={"tab-" + value} 
              className="tab-button" 
              type="radio" 
              // 同nameを持つradioボタンのグループ化
              name="tab" 
              value={value} 
              // 変更時処理
              onChange={changeDisplayFromTab} 
              // AppDisplayに渡すref
              ref={props.displayHandlers[value].tab} 
              defaultChecked={value === "main"}
            />
            <label className="tab-label" htmlFor={"tab-" + value}>
              {
                value === "main" 
                  ? "メイン" 
                  : value === "data" 
                  ? "データ" 
                  : value === "edit" 
                  ? "編集" 
                  : "設定"
              }
            </label>
          </div>
        )
      }
    </div>
  )
}

export default TabsWrapper;