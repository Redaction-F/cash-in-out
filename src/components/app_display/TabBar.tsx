import { DisplayHandler, Global } from "../../logic";
import { displayNames, DisplayName, isDisplayName } from "./logic";

// display群を切り替えるためのtab群
function TabBar(props: {
  displayHandlers: {[key in DisplayName]: DisplayHandler}, 
  global: Global
}) {
  // tabのonChangeからdisplay切り替え
  // onChangeに設定するHTMLInputElementのvalueはDisplayName型でないといけない
  const changeDisplayFromTab = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value: string = event.target.value;
    if (isDisplayName(value)) {
      props.global.changeDisplay!(value);
    } else {
      console.log("Developer Error: Tab name is invalid.");
    };
  }

  return (
    <div className="tabbar-container">
      {
        displayNames.map((value) => 
          <div className="tabbar-tab" key={value}>
            <input 
              // label関連付け用
              id={"tab-" + value} 
              className="tabbar-tab-button" 
              type="radio" 
              // 同nameを持つradioボタンのグループ化
              name="tab" 
              value={value} 
              // 変更時処理
              onChange={changeDisplayFromTab} 
              // AppDisplayに渡すref
              ref={props.displayHandlers[value].tab}
              // mainのみtrue、それ以外はfalse
              defaultChecked={value === "main"}
            />
            <label className="tabbar-tab-label" htmlFor={"tab-" + value}>
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

export default TabBar;