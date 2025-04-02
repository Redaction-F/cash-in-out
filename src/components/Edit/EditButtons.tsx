import { invoke } from "@tauri-apps/api";
import { CashIORecord, SpecialFunctions } from "../../logic";
import { EditFunctions, InputsFunctions, ModeOfEdit } from "./logic";

function EditButtons(props: {
  mode: ModeOfEdit, 
  setModeWrapper: (value: ModeOfEdit) => void, 
  editFunctions: EditFunctions, 
  inputsFunctions: InputsFunctions, 
  specialFunctions: SpecialFunctions
}) {
  // 出入金データ新規作成開始
  function startCreate() {
    props.setModeWrapper("createMode");
  }
  // 出入金データ編集開始
  async function startEdit(id: number | null) {
    if (id === null) {
      alert("IDを入力して下さい。");
      return;
    }
    let records = await invoke<CashIORecord | null>("get_record_by_id", {id: id})
    if (records === null) {
      alert("入力されたIDのデータは存在しません。\n存在するデータのIDを入力するか、データ一覧から選択して編集してください。");
    } else {
      props.inputsFunctions.set!(records);
      props.setModeWrapper("updateMode");
    }
  }
  // 出入金データ新規作成
  async function doCreate() {
    let newData: CashIORecord = props.inputsFunctions.get!();
    invoke<void>("create_record", {newRecord: newData}).then(() => {
      alert("新規作成が完了しました。");
      props.inputsFunctions.setEmpty!();
      props.setModeWrapper("selectMode");
    }, (e) => {
      console.log(e);
      alert("エラーが発生しました。エラーメッセージは以下の通りです。\n" + String(e));
    })
  }
  // 出入金データ更新
  async function doUpdate() {
    let changedData: CashIORecord = props.inputsFunctions.get!();
    invoke<void>("update_record", {changedRecord: changedData}).then(() => {
      alert("編集が完了しました。");
      props.inputsFunctions.setEmpty!();
      props.setModeWrapper("selectMode");
    }, (e) => {
      console.log(e);
      alert("エラーが発生しました。エラーメッセージは以下の通りです。\n" + String(e));
    })
  }
  // 出入金データ編集中止
  function cancelEdit() {
    props.inputsFunctions.setEmpty!();
    props.setModeWrapper("selectMode");
  }
  // IDフォームの内容から編集開始
  function startEditById() {
    startEdit(props.inputsFunctions.getId!())
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