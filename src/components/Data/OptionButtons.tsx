import { SpecialFunctions } from "../../logic";

function OptionButtons(props: {checkedCount: number, firstCheckedId: number | null, specialFunctions: SpecialFunctions}) {
  // 編集タブに遷移し、編集開始
  async function startEditWrap() {
    if (await props.specialFunctions.changeDisplay!("edit")) {
      props.specialFunctions.startEdit!(props.firstCheckedId);
    };
  }

  return(
    <div className="option-buttons">
      <button className="option-button" onClick={startEditWrap} disabled={props.checkedCount !== 1}>編集</button>
      <button className="option-button" disabled={props.checkedCount === 0}>削除</button>
    </div>
  )
}

export default OptionButtons;