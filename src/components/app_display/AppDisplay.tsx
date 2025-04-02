import { useRef } from "react";
import Displays from "./Displays";
import TabBar from "./TabBar";
import { DisplayHandler, SpecialFunctions } from "../../logic";
import { DisplayName, emptyHandler } from "./logic";

// 画面全体
function AppDisplay() {
  // display切り替え
  function changeDisplaySimple(preTab: DisplayName, nextTab: DisplayName) {
    // tabを切り替える
    displayHandlers[preTab].tab.current!.checked = false;
    displayHandlers[nextTab].tab.current!.checked = true;
    // 表示するdisplayを切り替える
    displayHandlers[preTab].content.current?.classList.remove("display-show");
    displayHandlers[nextTab].content.current?.classList.add("display-show");
  }
  // display切り替え(処理付き)
  async function changeDisplay(tabName: DisplayName): Promise<boolean> {
    // 切り替える必要がなければ終わる
    if (tabName == currentTab.current) {
      return false;
    }
    // 一度戻す
    changeDisplaySimple(tabName, currentTab.current);
    // close時の処理を実行、closeの許可が出るまで待機
    if (await displayHandlers[currentTab.current].onClose()) {
      // open時の処理を実行
      await displayHandlers[tabName].onOpen();
      // 切り替える
      changeDisplaySimple(currentTab.current, tabName);
      // 現在表示中のtabを更新
      currentTab.current = tabName;
      return true;
    } else {
      return false;
    };
  };
  
  // 現在表示中のdisplay
  const currentTab = useRef<DisplayName>("main");
  // 各displayのDisplayHandler
  const displayHandlers: {[key in DisplayName]: DisplayHandler} = {
    main: emptyHandler(), 
    data: emptyHandler(), 
    edit: emptyHandler(), 
    setting: emptyHandler()
  };
  // 全体共有用の関数群
  const specialFunctions: SpecialFunctions = {
    changeDisplay: undefined, 
    startEdit: undefined, 
    startCreate: undefined
  };
  
  // specicalFunctionを設定
  specialFunctions.changeDisplay = changeDisplay;

  return (
    <>
      {/* display群 */}
      <Displays displayHandlers={displayHandlers} specialFunctions={specialFunctions} />
      {/* tab群 */}
      <TabBar displayHandlers={displayHandlers} changeDisplay={changeDisplay} />
    </>
  )
}

export default AppDisplay;