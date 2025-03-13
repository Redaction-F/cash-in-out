import { useState } from "react";
import Displays from "./Displays";
import TabsWrapper from "./TabsWrapper";
import { DisplayHandler, SpecialFunctions } from "../../logic";
import { DisplayName, emptyHandler } from "./logic";

// 画面全体
function AppDisplay() {
  function changeDisplaySimple(preTab: DisplayName, nextTab: DisplayName) {
    // tabを切り替える
    displayHandlers[preTab].tab.current!.checked = false;
    displayHandlers[nextTab].tab.current!.checked = true;
    // 表示するdisplayを切り替える
    displayHandlers[preTab].content.current?.classList.remove("display-show");
    displayHandlers[nextTab].content.current?.classList.add("display-show");
  }
  // display切り替え用
  async function changeDisplay(tabName: DisplayName): Promise<boolean> {
    if (tabName == currentTab) {
      return false;
    }
    // 一度戻す
    changeDisplaySimple(tabName, currentTab);
    // close時の処理を実行、closeの許可が出るまで待機
    if (await displayHandlers[currentTab].onClose()) {
      // 切り替える
      changeDisplaySimple(currentTab, tabName);
      // open時の処理を実行
      displayHandlers[tabName].onOpen();
      // 現在表示中のtabを更新
      setCurrentTab(tabName);
      return true;
    } else {
      return false;
    };
  };
  
  // 現在表示中のdisplay
  // useState: displayの切り替え
  const [currentTab, setCurrentTab] = useState<DisplayName>("main");
  // 各displayのDisplayHandler
  const displayHandlers: {[key in DisplayName]: DisplayHandler} = {
    main: emptyHandler(), 
    data: emptyHandler(), 
    edit: emptyHandler(), 
    set: emptyHandler()
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
      <TabsWrapper displayHandlers={displayHandlers} changeDisplay={changeDisplay} />
    </>
  )
}

export default AppDisplay;