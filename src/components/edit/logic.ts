import { CashIORecord } from "../../logic";

// editタブのモード
export type ModeOfEdit = "selectMode" | "updateMode" | "createMode";

// Inputsの操作用関数群
export type InputsFunctions = {
  set: ((value: CashIORecord) => void) | undefined, 
  setEmpty: (() => void) | undefined, 
  getId: (() => number | null) | undefined, 
  get: (() => CashIORecord) | undefined, 
  reload: (() => Promise<void>) | undefined
}
// データの編集用関数群
export type EditFunctions = {
  startEditFromId: (() => void) | undefined, 
  cancelEdit: (() => void) | undefined
}