import { invoke } from "@tauri-apps/api";
import { CashRecord, SpecialFunctions } from "../../logic";
import { EditFunctions, InputGetterSetter, ModeOfEdit } from "./logic";

function EditButtons(props: {mode: ModeOfEdit, editFunctions: EditFunctions, setModeWrapper: (value: ModeOfEdit) => void, inputGetterSetter: InputGetterSetter, specialFunctions: SpecialFunctions}) {
  // 出入金データ新規作成開始
  function startCreate() {
    props.setModeWrapper("createMode");
  }
  // 出入金データ編集開始
  function startEdit(id: number | null) {
    if (id === null) {
      alert("IDを入力して下さい。");
      return;
    }
    invoke<CashRecord | null>("get_record_by_id", {id: id}).then((value: CashRecord | null) => {
      if (value === null) {
        alert("入力されたIDのデータは存在しません。\n存在するデータのIDを入力するか、データ一覧から選択して編集してください。");
      } else {
        props.inputGetterSetter.set!(value);
        props.setModeWrapper("updateMode");
      }
    })
  }
  // 出入金データ新規作成
  function doCreate() {
    let newData: CashRecord = props.inputGetterSetter.get!();
    invoke("create_record", {newRecord: newData}).then(() => {
      alert("新規作成が完了しました。");
      props.inputGetterSetter.setEmpty!();
      props.setModeWrapper("selectMode");
    }, (e) => {
      console.log(e);
      alert("エラーが発生しました。エラーメッセージは以下の通りです。\n" + String(e));
    })
  }
  // 出入金データ更新
  function doUpdate() {
    let changedData: CashRecord = props.inputGetterSetter.get!();
    invoke("update_record", {changedRecord: changedData}).then(() => {
      alert("編集が完了しました。");
      props.inputGetterSetter.setEmpty!();
      props.setModeWrapper("selectMode");
    }, (e) => {
      console.log(e);
      alert("エラーが発生しました。エラーメッセージは以下の通りです。\n" + String(e));
    })
  }
  // 出入金データ編集中止
  function cancelEdit() {
    props.inputGetterSetter.setEmpty!();
    props.setModeWrapper("selectMode");
  }
  // IDフォームの内容から編集開始
  function startEditById() {
    startEdit(props.inputGetterSetter.getId!())
  }

  // editFunctionsの設定
  props.editFunctions.startEditFromId = startEditById;
  props.editFunctions.cancelEdit = cancelEdit;
  // spectionFunctionを設定
  props.specialFunctions.startEdit = startEdit;
  props.specialFunctions.startCreate = startCreate;

  return (
    <div className="edit-buttons">
      <button type="button" className="edit-button" onClick={startEditById} disabled={props.mode !== "selectMode"}>編集</button>
      <button type="button" className="edit-button" onClick={() => {
        if (props.mode === "selectMode") {
          startCreate()
        } else if (props.mode === "updateMode") {
          doUpdate()
        } else if (props.mode === "createMode") {
          doCreate()
        }
      }}>
        {
          props.mode === "selectMode"
          ? "新規" 
          : props.mode === "updateMode"
          ? "編集"
          : "作成"
        }
      </button>
      <button type="button" className="edit-button" onClick={cancelEdit} disabled={props.mode === "selectMode"}>
        {
          props.mode === "selectMode"
          ? ""
          : "キャンセル"
        }
      </button>
    </div>
  )
}

export default EditButtons;