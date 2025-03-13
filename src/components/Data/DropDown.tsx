import { useRef, useState } from "react";
import { DropDownFunction, isMonthSelectType, MonthSelectType } from "./logic";

function DropDown(props: {dropDownFunction: DropDownFunction, setTableRowsFromMonth: (year: number, month: number) => void}) {
  // 選択されている年を取得
  function getYear(): number {
    let year = Number(yearSelect.current?.value)
    if (Number.isNaN(year)) {
      console.log("Developer error: selected value is not Year")
      return 0;
    };
    return year
  }
  // 選択されている月を取得
  function getMonth(): MonthSelectType {
    let month: number | null = monthSelect.current?.value === "null" ? null : Number(monthSelect.current?.value);
    if (isMonthSelectType(month)) {
      return month;
    } else {
      console.log("Developer error: selected value is not MonthSelect")
      return null;
    };
  }
  // 年変更時の処理
  function yearChanged() {
    setMonthSelectProperty({defalut: null, monthLen: yearSelect.current?.value === String(today.getFullYear()) ? today.getMonth() + 1 : 12});
  }
  // 月変更時の処理
  function updateTable() {
    let month: MonthSelectType = getMonth();
    if (month === null) {
      return;
    }
    props.setTableRowsFromMonth(getYear(), month);
  }

  // データ開始年
  const startYear: number = 2023;
  // 日付データ
  const today: Date = new Date();
  // 選択している年
  const yearSelect = useRef<HTMLSelectElement>(null);
  // 選択している月
  const monthSelect = useRef<HTMLSelectElement>(null);
  // 月のデフォルト値(更新すると再レンダリング)
  // useState: 月選択のドロップダウンの再レンダリング
  const [monthSelectProperty, setMonthSelectProperty] = useState<{defalut: MonthSelectType, monthLen: number}>(
    {defalut: MonthSelectType(today.getMonth() + 1), monthLen: today.getMonth() + 1}
  );
  // dropDownDunctionの初期化
  props.dropDownFunction.updateTable = updateTable;

  return (
    <div className="dropdown-month-wrapper">
      <select id="year" className="dropdown-month-select" onChange={yearChanged} defaultValue={today.getFullYear()} ref={yearSelect}>
        {
          Array(today.getFullYear() - startYear + 1).fill(0).map((_, i) => {
            let v = startYear + i;
            return <option value={v} key={v}>{String(v) + "年"}</option>
          })
        }
      </select>
      <select id="month" className="dropdown-month-select" onChange={updateTable} defaultValue={String(monthSelectProperty.defalut)} key={monthSelectProperty.defalut} ref={monthSelect}>
        <option value={"null"}>-</option>
        {
          Array(monthSelectProperty.monthLen).fill(0).map((_, i) => {
            return <option value={String(i + 1)} key={i}>{String(i + 1) + "月"}</option>
          })
        }
      </select>
    </div>
  )
}

export default DropDown;