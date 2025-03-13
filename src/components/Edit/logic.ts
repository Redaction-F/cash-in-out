import { CashRecord } from "../../logic";

// editタブのモード
export type ModeOfEdit = "selectMode" | "updateMode" | "createMode";
// editタブの入力の種類
export const inputKind = ["id", "date", "category", "title", "amount", "memo"] as const;
export type InputKind = typeof inputKind[number];
// InputKind判定
export function isInputKind(arg: string): arg is InputKind {
  return inputKind.some((value) => value === arg);
}
// editタブの入力の種類(memo除く)
export const inputKindWithoutMemo = ["id", "date", "category", "title", "amount"] as const;
export type InputKindWithoutMemo = typeof inputKindWithoutMemo[number];
// InputKindMemo判定
export function isInputKindWithoutMemo(arg: string): arg is InputKindWithoutMemo {
  return inputKind.some((value) => value === arg);
}
// Inputsの操作用関数群
export type InputGetterSetter = {
  set: ((value: CashRecord) => void) | undefined, 
  setEmpty: (() => void) | undefined, 
  getId: (() => number | null) | undefined, 
  get: (() => CashRecord) | undefined
}
// データの編集用関数群
export type EditFunctions = {
  startEditFromId: (() => void) | undefined, 
  cancelEdit: (() => void) | undefined
}