import { CashIORecord } from "../../logic";

// OptionButtons.tsxが提供する関数群
export type OptionButtonsRef = {
  onUpdateCheckBoxes: (checkCount: number) => void
};

// 期間を変更したときに実行する関数
export type OnTermChanged = (year: SelectYear, month: SelectMonth) => Promise<void>;

// TermSelect.tsxが提供する関数群
export type TermSelectRef = {
  reload: () => void
};

export type OnUpdateCheckBoxes = (checkBoxCount: number) => void;

// Table.tsxが提供する関数群
export type TableRef = {
  set: (value: CashIORecord[], sum: number) => void, 
  setByMonth: (year: SelectYear, month: SelectMonth) => Promise<void>, 
  getCheckedIds: () => number[], 
};

// 表の各行のチェック状態を管理
type CheckedState = {
  id: number, 
  isChecked: boolean
};

// 表のチェック状態を管理
export class CheckedStates {
  private _value: CheckedState[];
  private _checkedCount: number;

  constructor() {
    this._value = [];
    this._checkedCount = 0;
  }

  init(tableRows: CashIORecord[], onUpdateCheckBoxes: OnUpdateCheckBoxes | undefined) {
    this._value = tableRows.map((v) => ({
      id: v.id, 
      isChecked: false
    }));
    this._checkedCount = 0;
    if (onUpdateCheckBoxes === undefined) {
      return;
    }
    onUpdateCheckBoxes(0);
  }

  update(index: number, isChecked: boolean, onUpdateCheckBoxes: OnUpdateCheckBoxes | undefined) {
    // チェックが外されたとき
    if (this._value[index].isChecked && !isChecked) {
      this._value[index].isChecked = false;
      this._checkedCount -= 1;
      if (onUpdateCheckBoxes === undefined) {
        return;
      }
      onUpdateCheckBoxes(this._checkedCount);
    // チェックされたとき
    } else if (!this._value[index].isChecked && isChecked) {
      this._value[index].isChecked = true;
      this._checkedCount += 1;
      if (onUpdateCheckBoxes === undefined) {
        return;
      }
      onUpdateCheckBoxes(this._checkedCount);
    };
  }

  getCheckedIds(): number[] {
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
}