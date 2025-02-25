import { SpecialFunctions } from "../../logic";

function OptionButtons(props: {checkedCount: number, firstCheckedId: number, specialFunctions: SpecialFunctions}) {
  async function startEdit() {
    await props.specialFunctions.changeDisplay!("edit");
    props.specialFunctions.startEdit!(props.firstCheckedId);
  }

  return(
    <div className="option-buttons">
      <button className="option-button" onClick={startEdit} disabled={props.checkedCount !== 1}>編集</button>
      <button className="option-button" disabled={props.checkedCount === 0}>削除</button>
    </div>
  )
}

export default OptionButtons;