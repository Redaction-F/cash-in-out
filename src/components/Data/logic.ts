import { invoke } from "@tauri-apps/api";
import { CashIORecord } from "../../logic";

// Data.tsxが提供する関数群
export type dataFunctions = {
  init: (() => Promise<void>) | undefined
}

// Table.tsxが提供する関数群
export type TableFunctions = {
  set: ((value: CashIORecord[]) => void) | undefined, 
  setByMonth: ((year: SelectYear, month: SelectMonth) => Promise<void>) | undefined, 
  getCheckedId: (() => number[]) | undefined, 
}

// TermSelect.tsxが提供する関数群
export type TermSelectFunctions = {
  reload: (() => void) | undefined
}

// OptionButtons.tsxが提供する関数群
export type OptionButtonsFunctions = {
  clearCheckedCount: (() => void) | undefined, 
  incCheckedCount: (() => void) | undefined, 
  decCheckedCount: (() => void) | undefined
}

// 今月のデータを取得
export async function getCashIORecordInThisMonth(): Promise<CashIORecord[]> {
  let today: Date = new Date();
  return await invoke<CashIORecord[]>("get_records_by_month", {year: today.getFullYear(), month: today.getMonth() + 1});
}

// データを取得
export async function getCashIORecoByMonth(year: SelectYear, month: SelectMonth): Promise<CashIORecord[]> {
  if (month === null) {
    return [];
  } else {
    return await invoke<CashIORecord[]>("get_records_by_month", {year: year.value, month: month});
  }
}

// 表の各行のチェック状態を管理
type CheckedState = {
  id: number, 
  isChecked: boolean
}

// 表のチェック状態を管理
export class CheckedStates {
  private _value: CheckedState[];

  constructor() {
    this._value = [];
  }

  init(tableRows: CashIORecord[], optionButtonsFunctions: OptionButtonsFunctions) {
    this._value = tableRows.map((v) => ({
      id: v.id, 
      isChecked: false
    }));
    optionButtonsFunctions.clearCheckedCount!();
  }

  update(index: number, isChecked: boolean, optionButtonsFunctions: OptionButtonsFunctions) {
    // チェックが外されたとき
    if (this._value[index].isChecked && !isChecked) {
      this._value[index].isChecked = false;
      optionButtonsFunctions.decCheckedCount!();
    // チェックされたとき
    } else if (!this._value[index].isChecked && isChecked) {
      this._value[index].isChecked = true;
      optionButtonsFunctions.incCheckedCount!();
    };
  }

  getCheckedId(): number[] {
    return this._value.filter((v) => v.isChecked).map(v => v.id);
  }
}

// 選択できる月の型
const selectMonths = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, null] as const;
export type SelectMonth = typeof selectMonths[number];
const selectMonthsSet: Set<number | null> = new Set(selectMonths);
// MonthSelectTypeであるか判定
export function isSelectMonth(arg: number | null): arg is SelectMonth {
  return selectMonthsSet.has(arg);
}
// argがMonthSelectTypeならそのまま、そうでないならnullを返す
export function selectMonth(arg: number | null): SelectMonth {
  if (isSelectMonth(arg)) {
    return arg;
  } else {
    console.log("Developer error: The value is not SelectMonth.");
    return null;
  }
}

// 選択できる年の型
export class SelectYear {
  private static _startYear: number = 2023;
  private static _endYear: number = (new Date()).getFullYear();
  private _value: number;

  constructor(value: number) {
    if (SelectYear._startYear <= value && value <= SelectYear._endYear) {
      this._value = value;
    } else {
      console.log("Developer error: The value is not SelectYear.");
      this._value = SelectYear._endYear;
    }
  }

  isThisYear(): boolean {
    return this._value === SelectYear._endYear;
  }

  static yearArray(): number[] {
    return (new Array(this._endYear - this._startYear + 1)).fill(0).map((_, i) => this._startYear + i);
  }

  get value(): number {
    return this._value;
  }

  static get startYear(): SelectYear {
    return new SelectYear(SelectYear._startYear)
  }

  static get endYear(): SelectYear {
    return new SelectYear(SelectYear._endYear)
  }
}