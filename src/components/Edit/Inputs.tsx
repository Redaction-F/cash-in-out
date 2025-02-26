import { RefObject } from "react";
import { ModeOfEdit } from "../../logic";

function Inputs(props: {mode: ModeOfEdit, inputs: {[key: string]: RefObject<HTMLInputElement>}, inputMemo: RefObject<HTMLTextAreaElement>, inputDefaultValues: {[key: string]: string}}) {
  return (
    <div className="inputs-wrapper">
      {
        ["id", "date", "category", "title", "amount"].map((value) => 
          <div className="input-row">
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
              defaultValue={props.inputDefaultValues[value]} 
              // props.inputDefaultValues[value]の変化時に再レンダリングする用
              key={props.inputDefaultValues[value]} 
              ref={props.inputs[value]} 
              // valueが"id"のとき、props.mode === "selectMode"ならfalse、そうでないならtrue
              // valueが"id"でないとき、props.mode === "selectMode"ならtrue、そうでないならfalse
              disabled={(value === "id") !== (props.mode === "selectMode")} 
            />
          </div>
        )
      }
      <div className="input-row">
        <label className="input-label" htmlFor="memo">メモ</label>
        <textarea className="input-textarea" defaultValue={props.inputDefaultValues["memo"]} key={props.inputDefaultValues["memo"]} ref={props.inputMemo} disabled={props.mode !== "editMode"} />
      </div>
    </div>
  )
}

export default Inputs;