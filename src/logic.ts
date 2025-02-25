// .tsxファイルで用いる型の定義
// 具体的な機能はrustで書く
import { RefObject } from "react";

export type CashRecord = {
  id: number, 
  date: string, 
  category: string, 
  title: string, 
  amount: number, 
  memo: string
};

export type ModeOfEdit = "selectMode" | "editMode";
export type DisplayName = "main" | "data" | "edit" | "set";

export type DisplayHandler = {
  content: RefObject<HTMLDivElement>, 
  tab: RefObject<HTMLInputElement>, 
  onClose: () => Promise<boolean>, 
  onOpen: () => void
}

export type SpecialFunctions = {
  changeDisplay: ((displayName: DisplayName) => Promise<void>) | undefined
  startEdit: ((id: number) => void) | undefined
}