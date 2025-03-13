import { useState } from "react";
import EditButtons from "./EditButtons";
import Inputs from "./Inputs";
import { DisplayHandler, SpecialFunctions } from "../../logic";
import { EditFunctions, InputGetterSetter, ModeOfEdit } from "./logic";

// edit ディスプレイ
// 出入金レコードの入力受付
function Edit(props: {displayHandler: DisplayHandler, specialFunctions: SpecialFunctions}) {
  // modeの変更
  function setModeWrapper(value: ModeOfEdit) {
    setMode(value);
  }

  // 入力フォームの出入力管理
  const inputGetterSetter: InputGetterSetter = {
    set: undefined, 
    setEmpty: undefined, 
    getId: undefined, 
    get: undefined, 
  };  
  // 編集の管理
  const editFunctions: EditFunctions = {
    startEditFromId: undefined, 
    cancelEdit: undefined
  }
  // 現在のモード
  // useState: 入力フォームの再レンダリング
  const [mode, setMode] = useState<ModeOfEdit>("selectMode");
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
      <EditButtons mode={mode} editFunctions={editFunctions} setModeWrapper={setModeWrapper} inputGetterSetter={inputGetterSetter} specialFunctions={props.specialFunctions}/>
      <Inputs mode={mode} editFunctions={editFunctions} inputGetterSetter={inputGetterSetter}/>
    </>
  )
}

export default Edit;