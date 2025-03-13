import { RefObject } from "react";
import { DisplayName } from "./components/app_display/logic";

// .tsxファイルで用いる型の定義
// 具体的な機能はrustで書く

// 全体共有用の関数群
export type SpecialFunctions = {
  // display切り替え
  changeDisplay: ((displayName: DisplayName) => Promise<boolean>) | undefined, 
  // edit displayで編集を開始
  startEdit: ((id: number | null) => void) | undefined, 
  startCreate: (() => void) | undefined
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
