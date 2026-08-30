import { CashIORecord } from "../../logic";

// editタブのモード
export type ModeOfEdit = "select" | "update" | "create";

// Inputsの操作用関数群
export type InputsRef = {
  set: (value: CashIORecord) => void, 
  setEmpty: () => void, 
  getId: () => number | null, 
  get: () => CashIORecord, 
  reload: () => Promise<void>
};