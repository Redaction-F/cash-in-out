import { useState } from "react";
import { CashIORecord, SpecialFunctions } from "../../logic";
import { dataFunctions, OptionButtonsFunctions, TableFunctions } from "./logic";

// ボタン等
function OptionButtons(props: {
  dataFunctions: dataFunctions, 
  tableFunctions: TableFunctions, 
  optionButtonsFunctions: OptionButtonsFunctions, 
  specialFunctions: SpecialFunctions
}) {
  // 編集タブに遷移し、編集開始
  async function editWrap() {
    if (await props.specialFunctions.changeDisplay!("edit")) {
      let checkedId: number[] = props.tableFunctions.getCheckedId!();
      await props.specialFunctions.startEdit!(checkedId.length === 0 ? null : checkedId[0]);
    };
  }
  // 編集タブに遷移し、新規作成開始
  async function createWrap() {
    if (await props.specialFunctions.changeDisplay!("edit")) {
      props.specialFunctions.startCreate!();
    };
  }
  // データを削除
  async function deleteWrap() {
    if (!await confirm("削除しますか？")) {
      return;
    }
    let checkedId: number[] = props.tableFunctions.getCheckedId!();
    let errors: string[] = [];
    let isErrorOccured: boolean = false;
    for (let v of checkedId) {
      await CashIORecord.deleteById(v).then(() => {}, (e) => {
        isErrorOccured = true;
        errors.push(String(e));
      })
    }
    if (isErrorOccured) {
      console.log(errors.join("\n"));
      alert("エラーが発生しました。エラーメッセージは以下の通りです。 \n" + errors.join("\n"));
    } else {
      alert("データを削除しました。")
    }
    props.dataFunctions.reload!();
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
      <button type="button" className="option-button" onClick={createWrap} disabled={checkedCount !== 0}>新規</button>
      <button type="button" className="option-button" onClick={editWrap} disabled={checkedCount !== 1}>編集</button>
      <button type="button" className="option-button" onClick={deleteWrap} disabled={checkedCount === 0}>削除</button>
    </div>
  )
}

export default OptionButtons;