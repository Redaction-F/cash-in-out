import { SpecialFunctions } from "../../logic";

function OptionButtons(props: {checkedRowsCount: number, specialFunctions: SpecialFunctions}) {
  async function startEdit() {
    await props.specialFunctions.changeDisplay!("edit");
    props.specialFunctions.startEdit!("10000");
  }

  return(
    <div className="option-buttons">
      <button className="option-button" onClick={startEdit} disabled={props.checkedRowsCount !== 1}>編集</button>
      <button className="option-button" disabled={props.checkedRowsCount === 0}>削除</button>
    </div>
  )
}

export default OptionButtons;