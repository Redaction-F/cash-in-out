import { RefObject } from "react";
import { ModeOfEdit } from "../../logic";

function EditButtons(props: {startEdit: (id: number | null) => void, edit: () => void, cancelEdit: () => void, mode: ModeOfEdit, inputId: RefObject<HTMLInputElement>}) {
  // IDフォームの内容から編集開始
  function startEditWraper() {
    let id: string | undefined = props.inputId.current?.value;
    props.startEdit(id === "" ? null : Number(id))
  }

  return (
    <div className="edit-buttons">
      <button className="edit-button" onClick={startEditWraper} disabled={props.mode !== "selectMode"}>編集</button>
      <button className="edit-button" onClick={props.edit} disabled={props.mode !== "editMode"}>変更</button>
      <button className="edit-button" onClick={props.cancelEdit} disabled={props.mode !== "editMode"}>キャンセル</button>
    </div>
  )
}

export default EditButtons;