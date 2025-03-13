// 表の各行のチェック状態を管理
export type CheckedState = {
  id: number, 
  isChecked: boolean
}
// DropDown.tsxが提供する関数群
export type DropDownFunction = {
  updateTable: (() => void) | undefined
}
// 選択できる月の型
const monthSelectType = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, null] as const;
export type MonthSelectType = typeof monthSelectType[number];
// MonthSelectTypeであるか判定
export function isMonthSelectType(arg: number | null): arg is MonthSelectType {
  return monthSelectType.some((v) => v === arg);
}
// argがMonthSelectTypeならそのまま、そうでないならnullを返す
export function MonthSelectType(arg: number | null): MonthSelectType {
  if (isMonthSelectType(arg)) {
    return arg;
  } else {
    return null;
  }
}