import { RefObject, useEffect, useRef, useState } from "react";
import { CashRecord } from "../../logic";
import { EditFunctions, InputGetterSetter, InputKind, inputKind, inputKindWithoutMemo, InputKindWithoutMemo, ModeOfEdit } from "./logic";

function Inputs(props: {mode: ModeOfEdit, editFunctions: EditFunctions, inputGetterSetter: InputGetterSetter}) {
  // valueの内容に入力フォームを初期化
  function setInput(value: CashRecord) {
    inputDefaultValues["id"].current = String(value.id);
    inputDefaultValues["date"].current = value.date;
    inputDefaultValues["category"].current = value.category;
    inputDefaultValues["title"].current = value.title;
    inputDefaultValues["amount"].current = String(value.amount);
    inputDefaultValues["memo"].current = value.memo;
    setForceUpdateDefalut((prev) => 1 - prev);
  }
  // 入力フォームを初期化
  function setInputEmpty() {
    inputKind.forEach((value) => {
      inputDefaultValues[value].current = "";
    })
    setForceUpdateDefalut((prev) => 1 - prev);
  }
  // 入力されたIdの取得
  function getInputedId(): number | null {
    let id: string = inputs["id"].current!.value;
    return id === "" ? null : Number(id)
  }
  // 入力データの取得
  function getInput(): CashRecord {
    let id: string = inputs["id"].current!.value;
    let amount: string = inputs["amount"].current!.value;
    return {
      id: id === "" ? 0 : Number(id), 
      date: inputs["date"].current!.value, 
      category: inputs["category"].current!.value, 
      title: inputs["title"].current!.value, 
      amount: amount === "" ? 0 : Number(amount), 
      memo: inputMemo.current!.value
    }
  }

  // フォームの要素
  const inputs: {[key in InputKindWithoutMemo]: RefObject<HTMLInputElement>} = {
    id: useRef<HTMLInputElement>(null), 
    date: useRef<HTMLInputElement>(null), 
    category: useRef<HTMLInputElement>(null), 
    title: useRef<HTMLInputElement>(null), 
    amount: useRef<HTMLInputElement>(null), 
  };
  const inputMemo: RefObject<HTMLTextAreaElement> = useRef<HTMLTextAreaElement>(null);
  // フォームのデフォルトの値
  const inputDefaultValues: {[key in InputKind]: React.MutableRefObject<string>} = {
    id: useRef<string>(""), 
    date: useRef<string>(""), 
    category: useRef<string>(""), 
    title: useRef<string>(""), 
    amount: useRef<string>(""), 
    memo: useRef<string>("")
  };
  // フォームを強制再レンダリング
  const [forceUpdateDefault, setForceUpdateDefalut] = useState<number>(0);
  // 初回レンダリング判定
  const firstRender = useRef<boolean>(true);
  // inputGetterSetterの設定
  props.inputGetterSetter.set = setInput;
  props.inputGetterSetter.setEmpty = setInputEmpty;
  props.inputGetterSetter.getId = getInputedId;
  props.inputGetterSetter.get = getInput;

  // 
  useEffect(() => {
    if (firstRender.current) {
      inputs["id"].current?.addEventListener("keydown", (e) => {
        if (!e.isComposing && e.key === "Enter") {
          props.editFunctions.startEditFromId!();
        }
      })
      firstRender.current = false;
    }
  }, [])

  return (
    <div className="inputs-wrapper" key={forceUpdateDefault}>
      {
        inputKindWithoutMemo.map((value) => 
          <div className="input-row" key={value}>
            <label className="input-label" htmlFor={value}>
              {
                value === "id"
                ? "id"
                : value === "date"
                ? "日付"
                : value === "category"
                ? "カテゴリ"
                : value === "title"
                ? "タイトル"
                : "金額"
              }
            </label>
            <input 
              type={
                value === "id" || value == "amount"
                ? "number"
                : "text"
              } 
              id={value} 
              className="input-input" 
              defaultValue={inputDefaultValues[value].current} 
              ref={inputs[value]} 
              // valueが"id"のとき、props.mode === "selectMode"ならfalse、そうでないならtrue
              // valueが"id"でないとき、props.mode === "selectMode"ならtrue、そうでないならfalse
              disabled={value === "id" ? props.mode !== "selectMode" : props.mode === "selectMode"} 
            />
          </div>
        )
      }
      <div className="input-row">
        <label className="input-label" htmlFor="memo">メモ</label>
        <textarea 
          id="memo" 
          className="input-textarea" 
          defaultValue={inputDefaultValues["memo"].current} 
          ref={inputMemo} 
          disabled={props.mode === "selectMode"} 
        />
      </div>
    </div>
  )
}

export default Inputs;