import { useState } from "react";
import { SpecialFunctions } from "../../logic";
import { OptionButtonsFunctions, TableFunctions } from "./logic";

// ボタン等
function OptionButtons(props: {
  tableFunctions: TableFunctions, 
  optionButtonsFunctions: OptionButtonsFunctions, 
  specialFunctions: SpecialFunctions
}) {
  // 編集タブに遷移し、編集開始
  async function startEditWrap() {
    if (await props.specialFunctions.changeDisplay!("edit")) {
      await props.specialFunctions.startEdit!(props.tableFunctions.getFirstCheckedId!());
    };
  }
  // 編集タブに遷移し、新規作成開始
  async function startCreateWrap() {
    if (await props.specialFunctions.changeDisplay!("edit")) {
      props.specialFunctions.startCreate!();
    };
  }

  // 表でチェックされている行の数
  // useState: ボタンのdisabledの切り替え
  const [checkedCount, setCheckedCount] = useState<number>(0);

  // optionButtonsFucntionsの初期化
  props.optionButtonsFunctions.clearCheckedCount = () => setCheckedCount(0);
  props.optionButtonsFunctions.incCheckedCount = () => setCheckedCount((prev) => prev + 1);
  props.optionButtonsFunctions.decCheckedCount = () => setCheckedCount((prev) => prev - 1);

  return(
    <div className="option-buttons">
      <button type="button" className="option-button" onClick={startCreateWrap} disabled={checkedCount !== 0}>新規</button>
      <button type="button" className="option-button" onClick={startEditWrap} disabled={checkedCount !== 1}>編集</button>
      <button type="button" className="option-button" disabled={checkedCount === 0}>削除</button>
    </div>
  )
}

export default OptionButtons;