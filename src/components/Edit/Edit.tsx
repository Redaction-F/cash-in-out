import { useState } from "react";
import EditButtons from "./EditButtons";
import Inputs from "./Inputs";
import { DisplayHandler, SpecialFunctions } from "../../logic";
import { EditFunctions, InputsFunctions, ModeOfEdit } from "./logic";

// edit ディスプレイ
// 出入金レコードの入力受付
function Edit(props: {
  displayHandler: DisplayHandler, 
  specialFunctions: SpecialFunctions
}) {
  // modeの変更
  function setModeWrapper(value: ModeOfEdit) {
    setMode(value);
  }

  // 現在のモード
  // useState: 入力フォームの再レンダリング
  const [mode, setMode] = useState<ModeOfEdit>("selectMode");
  // 入力フォームの出入力管理
  const inputsFunctions: InputsFunctions = {
    set: undefined, 
    setEmpty: undefined, 
    getId: undefined, 
    get: undefined, 
    reload: undefined
  };  
  // 編集の管理
  const editFunctions: EditFunctions = {
    startEditFromId: undefined, 
    cancelEdit: undefined
  }

  // このdisplayに遷移時の処理
  props.displayHandler.onOpen = async(): Promise<void> => {
    await inputsFunctions.reload!();
  };
  // このdisplayから遷移時の処理
  props.displayHandler.onClose = async(): Promise<boolean> => {
    if (mode === "selectMode") {
      return true;
    }
    if (await confirm("編集を中止しますか？\n編集中のデータは破棄され、このデータは変更されません。")) {
      editFunctions.cancelEdit!();
      return true;
    } else {
      return false;
    }
  };
  
  return (
    <>
      <EditButtons mode={mode} setModeWrapper={setModeWrapper} editFunctions={editFunctions} inputsFunctions={inputsFunctions} specialFunctions={props.specialFunctions}/>
      <Inputs mode={mode} editFunctions={editFunctions} inputGetterSetter={inputsFunctions}/>
    </>
  )
}

export default Edit;