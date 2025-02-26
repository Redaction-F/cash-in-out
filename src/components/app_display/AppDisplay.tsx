import { useState, useRef } from "react";
import Displays from "./Displays";
import TabsWrapper from "./TabsWrapper";
import { DisplayHandler, DisplayName, SpecialFunctions } from "../../logic";

// 画面全体
function AppDisplay() {
  // デフォルトのHandler
  function emptyHandler(): DisplayHandler {
    return {
      content: useRef<HTMLDivElement>(null),
      tab: useRef<HTMLInputElement>(null), 
      onClose: async() => {
        return true
      },
      onOpen: () => {},
    };
  }
  // display切り替え用
  async function changeDisplay(tabName: DisplayName): Promise<boolean> {
    if (tabName == currentTab) {
      return false;
    }
    // 一度tabの切り替えを戻す
    displayHandlers[tabName].tab.current!.checked = false;
    displayHandlers[currentTab].tab.current!.checked = true;
    // close時の処理を実行、closeの許可が出るまで待機
    if (await displayHandlers[currentTab].onClose()) {
      // tabを切り替える
      displayHandlers[currentTab].tab.current!.checked = false;
      displayHandlers[tabName].tab.current!.checked = true;
      // 表示するdisplayを切り替える
      displayHandlers[currentTab].content.current?.classList.remove("display-show");
      displayHandlers[tabName].content.current?.classList.add("display-show");
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
  const [currentTab, setCurrentTab] = useState<DisplayName>("main");
  // 各displayのDisplayHandler
  const displayHandlers: {[key in DisplayName]: DisplayHandler} = {
    "main": emptyHandler(), 
    "data": emptyHandler(), 
    "edit": emptyHandler(), 
    "set": emptyHandler()
  };
  // 全体共有用の関数群
  const specialFunctions: SpecialFunctions = {
    changeDisplay: changeDisplay, 
    startEdit: undefined
  };

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