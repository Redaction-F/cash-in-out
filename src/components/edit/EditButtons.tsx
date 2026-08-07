import { forwardRef } from "react";
import { ModeOfEdit } from "./logic";

// 編集に関するボタン群
const EditButtons = forwardRef((props: {
  mode: ModeOfEdit,
  onStartEdit: () => void,
  onEdit: () => Promise<void>,
  onStartCreate: () => void,
  onCreate: () => Promise<void>,
  onCancel: () => void,
}, _ref) => {
  return (
    <div className="edit-buttons">
      <button type="button" className="edit-button" onClick={() => {
        if (props.mode === "select") {
          props.onStartEdit()
        } else if (props.mode === "update") {
          props.onEdit()
        } else if (props.mode === "create") {
          props.onCreate()
        }
      }} disabled={props.mode === "create"}>
        {
          props.mode === "select"
          ? "編集" 
          : props.mode === "update"
          ? "編集"
          : ""
        }
      </button>
      <button type="button" className="edit-button" onClick={() => {
        if (props.mode === "select") {
          props.onStartCreate()
        } else if (props.mode === "update") {
          props.onEdit()
        } else if (props.mode === "create") {
          props.onCreate()
        }
      }} disabled={props.mode === "update"}>
        {
          props.mode === "select"
          ? "新規" 
          : props.mode === "update"
          ? ""
          : "作成"
        }
      </button>
      <button type="button" className="edit-button" onClick={props.onCancel} disabled={props.mode === "select"}>
        {
          props.mode === "select"
          ? ""
          : "キャンセル"
        }
      </button>
    </div>
  )
})

export default EditButtons;