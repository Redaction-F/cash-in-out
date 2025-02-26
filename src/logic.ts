import { RefObject } from "react";

// .tsxファイルで用いる型の定義
// 具体的な機能はrustで書く

// displayの種類
export const displayName = ["main", "data", "edit", "set"] as const;
export type DisplayName = typeof displayName[number];
export function isDisplayName(arg: string): arg is DisplayName {
  return displayName.some((value) => value === arg);
}
// displayの操作用
export type DisplayHandler = {
  // displayであるdivタグの要素
  content: RefObject<HTMLDivElement>, 
  // displayを操作するtab
  tab: RefObject<HTMLInputElement>, 
  // このdisplayから遷移するときの処理
  // 返り値は遷移可能か否か
  onClose: () => Promise<boolean>, 
  // このdisplayに線にするときの処理
  onOpen: () => void
}
// 全体共有用の関数群
export type SpecialFunctions = {
  // display切り替え
  changeDisplay: ((displayName: DisplayName) => Promise<boolean>) | undefined, 
  // edit displayで編集を開始
  startEdit: ((id: number | null) => void) | undefined
}

// 出入金1単位
export type CashRecord = {
  id: number, 
  date: string, 
  category: string, 
  title: string, 
  amount: number, 
  memo: string
};

// editタブのモード
export type ModeOfEdit = "selectMode" | "editMode";