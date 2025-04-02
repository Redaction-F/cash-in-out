import { RefObject, useEffect, useRef, useState } from "react";
import MCategorySelect from "../setting/MCategorySelect";
import SCategorySelect from "../setting/SCategorySelect";
import { CashIORecord, CashIORecordField, cashIORecordFields } from "../../logic";
import { MCategorySelectFunctions, SelectMCategory, SelectSCategory, SCategorySelectFunctions } from "../setting/logic";
import { EditFunctions, InputsFunctions, ModeOfEdit } from "./logic";

function Inputs(props: {
  mode: ModeOfEdit, 
  editFunctions: EditFunctions, 
  inputGetterSetter: InputsFunctions
}) {
  // valueの内容に入力フォームを初期化
  function setInput(value: CashIORecord) {
    inputDefaultValues["id"].current = String(value.id);
    inputDefaultValues["date"].current = value.date;
    inputDefaultValues["mainCategory"].current = value.mainCategory;
    inputDefaultValues["subCategory"].current = value.subCategory;
    inputDefaultValues["title"].current = value.title;
    inputDefaultValues["amount"].current = String(value.amount);
    inputDefaultValues["memo"].current = value.memo;
    setRenderInput(1);
  }
  // 入力フォームを初期化
  function setInputEmpty() {
    cashIORecordFields.forEach((value) => {
      inputDefaultValues[value].current = "";
    })
    setRenderInput(1);
  }
  // 入力されたIdの取得
  function getInputedId(): number | null {
    let id: string = inputs["id"].current!.value;
    return id === "" ? null : Number(id)
  }
  // 入力データの取得
  function getInput(): CashIORecord {
    let id: string = inputs["id"].current!.value;
    let mainCategory: SelectMCategory | string = mainCategorySelectorFunctions.get!();
    let subCategory: SelectSCategory | string = subCategorySelectorFunctions.get!();
    let amount: string = inputs["amount"].current!.value;
    return {
      id: id === "" ? 0 : Number(id), 
      date: inputs["date"].current!.value, 
      mainCategory: (mainCategory instanceof SelectMCategory) ? mainCategory.value : mainCategory, 
      subCategory: (subCategory instanceof SelectSCategory) ? subCategory.value : subCategory, 
      title: inputs["title"].current!.value, 
      amount: amount === "" ? 0 : Number(amount), 
      memo: inputs["memo"].current!.value
    }
  }

  // フォームの要素
  const inputs: {
    id: RefObject<HTMLInputElement>, 
    date: RefObject<HTMLInputElement>, 
    title: RefObject<HTMLInputElement>, 
    amount: RefObject<HTMLInputElement>, 
    memo: RefObject<HTMLTextAreaElement>, 
  } = {
    id: useRef<HTMLInputElement>(null), 
    date: useRef<HTMLInputElement>(null), 
    title: useRef<HTMLInputElement>(null), 
    amount: useRef<HTMLInputElement>(null), 
    memo: useRef<HTMLTextAreaElement>(null), 
  };
  // フォームのデフォルトの値
  const inputDefaultValues: {[key in CashIORecordField]: React.MutableRefObject<string>} = {
    id: useRef<string>(""), 
    date: useRef<string>(""), 
    mainCategory: useRef<string>(""), 
    subCategory: useRef<string>(""), 
    title: useRef<string>(""), 
    amount: useRef<string>(""), 
    memo: useRef<string>("")
  };
  // MainCategorySelectorFunctions.tsxが提供する関数群
  const mainCategorySelectorFunctions: MCategorySelectFunctions = {
    get: undefined, 
    reload: undefined
  };
  // SubCategorySelectorFunctions.tsxが提供する関数群
  const subCategorySelectorFunctions: SCategorySelectFunctions = {
    get: undefined, 
    update: undefined
  };
  // フォームを再レンダリング
  const [renderInput, setRenderInput] = useState<number>(0);
  // 最初のレンダリングを判定
  const firstRender = useRef<boolean>(true);

  // inputGetterSetterの設定
  props.inputGetterSetter.set = setInput;
  props.inputGetterSetter.setEmpty = setInputEmpty;
  props.inputGetterSetter.getId = getInputedId;
  props.inputGetterSetter.get = getInput;
  props.inputGetterSetter.reload = () => mainCategorySelectorFunctions.reload!();
  // enterキーにバインド
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
  // select要素のために必ず二回レンダリング
  useEffect(() => {
    if (renderInput === 1) {
      setRenderInput(0);
    }
  }, [renderInput])

  return (
    <div className="inputs-wrapper" key={renderInput}>
      {
        cashIORecordFields.map((value) => 
          <div className="input-row" key={value}>
            <label className="input-label" htmlFor={value}>
              {
                value === "id"
                ? "id"
                : value === "date"
                ? "日付"
                : value === "mainCategory"
                ? "メインカテゴリ"
                : value === "subCategory"
                ? "サブカテゴリ"
                : value === "title"
                ? "タイトル"
                : value === "amount"
                ? "金額"
                : "メモ"
              }
            </label>
            {
              value === "id" || value === "date" || value === "title" || value === "amount"
              ? <input 
                type={
                  value === "id" || value === "amount"
                  ? "number"
                  : value === "date"
                  ? "date"
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
              : value === "mainCategory" 
              ? <MCategorySelect 
                mCategoryFunctions={mainCategorySelectorFunctions} 
                sCategoryFunctions={subCategorySelectorFunctions} 
                additionalOption={[]} 
                disabled={props.mode === "selectMode"} 
                defaultValue={inputDefaultValues[value].current}
              />
              : value === "subCategory"
              ? <SCategorySelect 
                mCategoryFunctions={mainCategorySelectorFunctions} 
                sCategoryFunctions={subCategorySelectorFunctions} 
                additionalOption={[]} 
                disabled={props.mode === "selectMode"} 
                defaultValue={inputDefaultValues[value].current}
              />
              : <textarea 
                id="memo" 
                className="input-textarea" 
                defaultValue={inputDefaultValues[value].current} 
                ref={inputs[value]} 
                disabled={props.mode === "selectMode"} 
              />
            }
          </div>
        )
      }
    </div>
  )
}

export default Inputs;