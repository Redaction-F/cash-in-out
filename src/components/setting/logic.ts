import { MCategory, SCategory } from "../../logic"

// Category.tsxが提供する機能群
export type CategoryFunctions = {
  reloadCategorySetting: (() => Promise<void>) | undefined
}

// MCategorySelect.tsxが提供する関数群
export type MCategorySelectFunctions = {
  get: (() => MCategory | string) | undefined, 
  reload: (() => Promise<void>) | undefined
}
// SCategorySelect.tsxが提供する関数群
export type SCategorySelectFunctions = {
  get: (() => SCategory | string) | undefined, 
  update: (() => void) | undefined, 
}
