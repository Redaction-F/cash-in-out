import { SpecialFunctions } from "../../logic";

// ボタン等
function OptionButtons(props: {checkedCount: number, getFirstCheckedId: () => number | null, specialFunctions: SpecialFunctions}) {
  // 編集タブに遷移し、編集開始
  async function startEditWrap() {
    if (await props.specialFunctions.changeDisplay!("edit")) {
      props.specialFunctions.startEdit!(props.getFirstCheckedId());
    };
  }
  // 編集タブに遷移し、新規作成開始
  async function startCreateWrap() {
    if (await props.specialFunctions.changeDisplay!("edit")) {
      props.specialFunctions.startCreate!();
    };
  }

  return(
    <div className="option-buttons">
      <button type="button" className="option-button" onClick={startCreateWrap} disabled={props.checkedCount !== 0}>新規</button>
      <button type="button" className="option-button" onClick={startEditWrap} disabled={props.checkedCount !== 1}>編集</button>
      <button type="button" className="option-button" disabled={props.checkedCount === 0}>削除</button>
    </div>
  )
}

export default OptionButtons;