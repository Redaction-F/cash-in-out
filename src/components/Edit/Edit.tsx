import { invoke } from "@tauri-apps/api/tauri";
import { RefObject, useRef, useState } from "react";
import EditButtons from "./EditButtons";
import { CashRecord, DisplayHandler, ModeOfEdit, SpecialFunctions } from "../../logic";

function Edit(props: {displayHandler: DisplayHandler, specialFunctions: SpecialFunctions}) {
  // valueの内容に入力フォームを初期化
  function setInput(value: CashRecord) {
    setInputDefaultValues["id"](String(value.id));
    setInputDefaultValues["date"](value.date);
    setInputDefaultValues["category"](value.category);
    setInputDefaultValues["title"](value.title);
    setInputDefaultValues["amount"](String(value.amount));
    setInputDefaultValues["memo"](value.memo);
  }
  // 入力フォームを初期化
  function setInputEmpty() {
    setInputDefaultValues["id"]("");
    setInputDefaultValues["date"]("");
    setInputDefaultValues["category"]("");
    setInputDefaultValues["title"]("");
    setInputDefaultValues["amount"]("");
    setInputDefaultValues["memo"]("");
  }
  // 出入金データ編集開始
  function startEdit(id: number | null) {
    if (id === null) {
      alert("IDを入力して下さい。");
      return;
    }
    invoke<CashRecord | null>("get_one_from_db", {id: id}).then((value: CashRecord | null) => {
      if (value === null) {
        alert("入力されたIDのデータは存在しません。\n存在するデータのIDを入力するか、データ一覧から選択して編集してください。");
      } else {
        setInput(value);
        setMode("editMode");
      }
    })
  }
  // 出入金データ更新
  function edit() {
    let changedData: CashRecord = {
      id: parseInt(inputs["id"].current?.value!), 
      date: inputs["date"].current?.value!, 
      category: inputs["category"].current?.value!, 
      title: inputs["title"].current?.value!, 
      amount: parseInt(inputs["amount"].current?.value!), 
      memo: inputMemo.current?.value!
    }
    invoke("update_one_from_db", {changedRecord: changedData}).then(() => {
      alert("編集が完了しました。");
      setInputEmpty();
      setMode("selectMode");
    }, (e) => {
      console.log(e);
      alert("エラーが発生しました。エラーメッセージは以下の通りです。\n" + String(e));
    })
  }
  // 出入金データ編集中止
  function cancelEdit() {
    setInputEmpty();
    setMode("selectMode");
  }
  
  // フォームの要素
  const inputs: {[key: string]: RefObject<HTMLInputElement>} = {};
  ["id", "date", "category", "title", "amount"].forEach((value: string) => {
    inputs[value] = useRef<HTMLInputElement>(null);
  })
  const inputMemo: RefObject<HTMLTextAreaElement> = useRef<HTMLTextAreaElement>(null);
  // フォームのデフォルトの値
  const inputDefaultValues: {[key: string]: string} = {};
  const setInputDefaultValues: {[key: string]: React.Dispatch<React.SetStateAction<string>>} = {};
  ["id", "date", "category", "title", "amount", "memo"].forEach((value: string) => {
    [inputDefaultValues[value], setInputDefaultValues[value]] = useState<string>("");
  })
  // 現在のモード
  const [mode, setMode] = useState<ModeOfEdit>("selectMode");
  // このdisplayから遷移時の処理
  props.displayHandler.onClose = async(): Promise<boolean> => {
    if (mode === "selectMode") {
      return true;
    }
    if (await confirm("編集を中止しますか？\n編集中のデータは破棄され、このデータは変更されません。")) {
      cancelEdit();
      return true;
    } else {
      return false;
    }
  };
  // 出入金データ編集開始
  props.specialFunctions.startEdit = startEdit;
  
  return (
    <>
      <EditButtons startEdit={startEdit} edit={edit} cancelEdit={cancelEdit} mode={mode} inputId={inputs["id"]}/>
      <div className="inputs-wrapper">
        <div className="input-row">
          <label className="input-label" htmlFor="id">id</label>
          <input type="number" id="id" className="input-input" defaultValue={inputDefaultValues["id"]} key={inputDefaultValues["id"]} ref={inputs["id"]} disabled={mode !== "selectMode"} />
        </div>
        <div className="input-row">
          <label className="input-label" htmlFor="date">日時</label>
          <input type="text" id="date" className="input-input" defaultValue={inputDefaultValues["date"]} key={inputDefaultValues["date"]} ref={inputs["date"]} disabled={mode !== "editMode"} />
        </div>
        <div className="input-row">
          <label className="input-label" htmlFor="category">カテゴリ</label>
          <input type="text" id="category" className="input-input" defaultValue={inputDefaultValues["category"]} key={inputDefaultValues["category"]} ref={inputs["category"]} disabled={mode !== "editMode"} />
        </div>
        <div className="input-row">
          <label className="input-label" htmlFor="title">タイトル</label>
          <input type="text" id="title" className="input-input" defaultValue={inputDefaultValues["title"]} key={inputDefaultValues["title"]} ref={inputs["title"]} disabled={mode !== "editMode"} />
        </div>
        <div className="input-row">
          <label className="input-label" htmlFor="amount">金額</label>
          <input type="number" id="amount" className="input-input" defaultValue={inputDefaultValues["amount"]} key={inputDefaultValues["amount"]} ref={inputs["amount"]} disabled={mode !== "editMode"} />
        </div>
        <div className="input-row">
          <label className="input-label" htmlFor="memo">メモ</label>
          <textarea className="input-textarea" defaultValue={inputDefaultValues["memo"]} key={inputDefaultValues["memo"]} ref={inputMemo} disabled={mode !== "editMode"} />
        </div>
      </div>
    </>
  )
}

export default Edit;