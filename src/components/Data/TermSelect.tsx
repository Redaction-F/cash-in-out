import { useRef, useState } from "react";
import { selectMonth, SelectMonth, SelectYear, TableFunctions, TermSelectFunctions } from "./logic";

function TermSelect(props: {
  tableFunctions: TableFunctions, 
  termSelectFunctions: TermSelectFunctions
}) {
  // 選択されている年を取得
  function getYear(): SelectYear {
    let year: number = Number(yearSelect.current?.value);
    return new SelectYear(year);
  }
  // 選択されている月を取得
  function getMonth(): SelectMonth {
    let month: number | null = monthSelect.current?.value === "null" ? null : Number(monthSelect.current?.value);
    return selectMonth(month);
  }
  function setMonthLen(value: number) {
    monthLen.current = value;
    setMonthRenderSelect((prev) => 1 - prev);
  }
  // 年変更時の処理
  function onUpdateOfYear() {
    setMonthLen(getYear().isThisYear() ? today.getMonth() + 1 : 12);
  }
  // 月変更時の処理
  async function onUpdateOfMonth() {
    let month: SelectMonth = getMonth();
    if (month === null) {
      return;
    }
    await props.tableFunctions.setTableRowsByMonth!(getYear(), month);
  }

  // 日付データ
  const today: Date = new Date();
  // 月のデフォルト値(更新すると再レンダリング)
  // useState: 月選択のドロップダウンの再レンダリング
  const monthLen = useRef<number>(today.getMonth() + 1);
  // 年選択
  const yearSelect = useRef<HTMLSelectElement>(null);
  // 月選択
  const monthSelect = useRef<HTMLSelectElement>(null);
  // 初期化時に更新
  // useState: ドロップダウンの再レンダリング
  const [renderYearSelect, setYearRenderSelect] = useState<number>(0);
  const [renderMonthSelect, setMonthRenderSelect] = useState<number>(0);

  // termSelectFunctionsの初期化
  props.termSelectFunctions.init = () => {
    setYearRenderSelect((prev) => 1 - prev);
    setMonthLen(today.getMonth() + 1);
  };

  return (
    <div className="dropdown-term-wrapper">
      <select 
        id="year" 
        className="dropdown-term-select" 
        onChange={onUpdateOfYear} 
        defaultValue={today.getFullYear()} 
        key={renderYearSelect} 
        ref={yearSelect}
      >
        {
          SelectYear.yearArray().map((v) => <option value={v} key={v}>{String(v) + "年"}</option>)
        }
      </select>
      <select 
        id="month" 
        className="dropdown-term-select" 
        onChange={onUpdateOfMonth} 
        defaultValue={String(monthLen.current)} 
        key={renderMonthSelect + 2} 
        ref={monthSelect}
      >
        <option value={"null"}>-</option>
        {
          (new Array(monthLen.current)).fill(0).map((_, i) => <option value={String(i + 1)} key={i}>{String(i + 1) + "月"}</option>)
        }
      </select>
    </div>
  )
}

export default TermSelect;