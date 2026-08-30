import { useRef } from "react";
import Displays from "./Displays";
import TabBar from "./TabBar";
import { DisplayHandler, Global } from "../../logic";
import { DisplayName, emptyHandler, LoadingDisplayRef } from "./logic";
import LoadingDisplay from "./LoadingDisplay";

// 画面全体
function AppDisplay() {
  // display切り替え
  const changeDisplaySimple = (presentTab: DisplayName, nextTab: DisplayName) => {
    // tabを切り替える
    displayHandlers[presentTab].tab.current!.checked = false;
    displayHandlers[nextTab].tab.current!.checked = true;
    // 表示するdisplayを切り替える
    displayHandlers[presentTab].content.current?.classList.remove("display-show");
    displayHandlers[nextTab].content.current?.classList.add("display-show");
  };
  // display切り替え(処理付き)
  const changeDisplay = async (tabName: DisplayName): Promise<boolean> => {
    // 切り替える必要がなければ終わる
    if (tabName == currentTab.current) {
      return false;
    }
    // ロード画面を表示
    loadingDisplayRef.current!.start();
    // 一度戻す
    changeDisplaySimple(tabName, currentTab.current);

    // close時の処理を実行、closeの許可が出るまで待機
    const isClosed = await displayHandlers[currentTab.current].onClose();
    if (!isClosed) {
    // ロード画面を消す
      loadingDisplayRef.current!.end();
      return false;
    }

    // open時の処理を実行
    await displayHandlers[tabName].onOpen();
    // 切り替える
    changeDisplaySimple(currentTab.current, tabName);
    // 現在表示中のtabを更新
    currentTab.current = tabName;
    // ロード画面を消す
    loadingDisplayRef.current!.end();
    return true;
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
  const global: Global = {
    changeDisplay: undefined, 
    startEdit: undefined, 
    startCreate: undefined
  };
  // ロード画面の機能の関数群
  const loadingDisplayRef = useRef<LoadingDisplayRef>(null);
  
  // specicalFunctionを設定
  global.changeDisplay = changeDisplay;

  return (
    <>
      <LoadingDisplay ref={loadingDisplayRef}/>
      {/* display群 */}
      <Displays displayHandlers={displayHandlers} global={global} />
      {/* tab群 */}
      <TabBar displayHandlers={displayHandlers} global={global} />
    </>
  )
}

export default AppDisplay;