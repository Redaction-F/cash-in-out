import Category from "./Category";
import { DisplayHandler } from "../../logic";
import { CategoryFunctions } from "./logic";

// setting ディスプレイ
// 諸設定
function Setting(props: {
  displayHandler: DisplayHandler
}) {
  // Category.tsxが提供する関数群
  const categoryFunctions: CategoryFunctions = {
    reloadCategorySetting: undefined
  }

  // settingのdisplayHandlerの初期化
  props.displayHandler.onOpen = async () => {
    categoryFunctions.reloadCategorySetting!()
  };

  return (
    <>
      <Category categoryFunction={categoryFunctions}/>
    </>
  )
}

export default Setting;