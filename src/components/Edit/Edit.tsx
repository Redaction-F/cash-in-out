import { invoke } from "@tauri-apps/api/tauri";
import { RefObject, useRef, useState } from "react";
import { CashRecord, DisplayHandler, ModeOfEdit, SpecialFunctions } from "../../logic";

function Edit(props: {displayHandler: DisplayHandler, specialFunctions: SpecialFunctions}) {
  const inputs: {[key: string]: RefObject<HTMLInputElement>} = {};
  ["id", "date", "category", "title", "amount"].forEach((value: string) => {
    inputs[value] = useRef<HTMLInputElement>(null);
  })
  const inputMemo: RefObject<HTMLTextAreaElement> = useRef<HTMLTextAreaElement>(null);
  const inputDefaultValues: {[key: string]: string} = {};
  const setInputDefaultValues: {[key: string]: React.Dispatch<React.SetStateAction<string>>} = {};
  ["id", "date", "category", "title", "amount", "memo"].forEach((value: string) => {
    [inputDefaultValues[value], setInputDefaultValues[value]] = useState<string>("");
  })
  const [mode, setMode] = useState<ModeOfEdit>("selectMode");

  props.displayHandler.onClose = async() => {
    if (mode === "selectMode") {
      return true;
    }
    if (await confirm("Cancel the edit? The data you are editting will not be saved.")) {
      cancelEdit();
      return true;
    } else {
      return false;
    }
  };

  function setInput(value: CashRecord) {
    setInputDefaultValues["id"](String(value.id));
    setInputDefaultValues["date"](value.date);
    setInputDefaultValues["category"](value.category);
    setInputDefaultValues["title"](value.title);
    setInputDefaultValues["amount"](String(value.amount));
    setInputDefaultValues["memo"](value.memo);
  }

  function setInputEmpty() {
    setInputDefaultValues["id"]("");
    setInputDefaultValues["date"]("");
    setInputDefaultValues["category"]("");
    setInputDefaultValues["title"]("");
    setInputDefaultValues["amount"]("");
    setInputDefaultValues["memo"]("");
  }

  function startEdit(id: number) {
    if (Number.isNaN(id)) {
      alert("Please input id.");
      return;
    }
    invoke<CashRecord | null>("get_one_from_db", {id: id}).then((value: CashRecord | null) => {
      if (value === null) {
        alert("There is no data of the inputted id.");
      } else {
        setInput(value);
        setMode("editMode");
      }
    })
  }
  props.specialFunctions.startEdit = startEdit;

  function edit() {
    let changedData: CashRecord = {
      id: parseInt(inputs["id"].current?.value!), 
      date: inputs["date"].current?.value!, 
      category: inputs["category"].current?.value!, 
      title: inputs["title"].current?.value!, 
      amount: parseInt(inputs["amount"].current?.value!), 
      memo: inputMemo.current?.value!
    }
    console.log(changedData.memo);
    invoke("update_one_from_db", {changedRecord: changedData}).then(() => {
      alert("Edit is completed.");
      setInputEmpty();
      setMode("selectMode");
    }, (e) => {
      console.log(e);
      alert("An error is occured: " + String(e));
    })
  }

  function cancelEdit() {
    setInputEmpty();
    setMode("selectMode");
  }
  
  return (
    <>
      <div className="edit-buttons">
        <button className="edit-button" onClick={startEdit.bind(window, Number(inputs["id"].current?.value))} disabled={mode !== "selectMode"}>編集</button>
        <button className="edit-button" onClick={edit} disabled={mode != "editMode"}>変更</button>
        <button className="edit-button" onClick={cancelEdit} disabled={mode != "editMode"}>キャンセル</button>
      </div>
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