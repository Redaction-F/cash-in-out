import { forwardRef, useImperativeHandle, useRef, useState } from "react";
import { OnTermChanged, selectMonth, SelectMonth, SelectYear, TermSelectRef } from "./logic";

const TermSelect = forwardRef((props: {
  onTermChanged: OnTermChanged | undefined
}, ref: React.ForwardedRef<TermSelectRef>) => {
  // 選択されている年を取得
  const getYear = (): SelectYear => {
    let year: number = Number(yearSelect.current?.value);
    return new SelectYear(year);
  };
  // 選択されている月を取得
  const getMonth = (): SelectMonth => {
    let month: number | null = monthSelect.current?.value === "null" ? null : Number(monthSelect.current?.value);
    return selectMonth(month);
  };
  // 最終月を更新
  const setMonthLen = (value: number) => {
    monthLen.current = value;
    setMonthRenderSelect((prev) => 1 - prev);
  };
  // 年変更時の処理
  const onUpdateOfYear = () => {
    setMonthLen(getYear().isThisYear() ? today.getMonth() + 1 : 12);
  };
  // 月変更時の処理
  const onUpdateOfMonth = async () => {
    let month: SelectMonth = getMonth();
    if (month === null) {
      return;
    }
    await props.onTermChanged!(getYear(), month);
  };

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

  // termSelectRefの初期化
  useImperativeHandle(ref, () => ({
    reload: () => {
      setYearRenderSelect((prev) => 1 - prev);
      setMonthLen(today.getMonth() + 1);
    }
  }));

  return (
    <div className="termselect-container">
      {/* 兄弟要素とkeyが被らないようにすみわけ */}
      <div className="all-select-wapper">
        <select 
          id="year"
          onChange={onUpdateOfYear} 
          defaultValue={today.getFullYear()} 
          key={renderYearSelect} 
          ref={yearSelect}
        >
          {
            SelectYear.yearArray().map((v) => <option value={v} key={v}>{String(v) + "年"}</option>)
          }
        </select>
      </div>
      {/* 兄弟要素とkeyが被らないようにすみわけ */}
      <div className="all-select-wapper">
        <select 
          id="month"
          onChange={onUpdateOfMonth} 
          defaultValue={String(monthLen.current)} 
          key={renderMonthSelect} 
          ref={monthSelect}
        >
          <option value={"null"}>-</option>
          {
            (new Array(monthLen.current)).fill(0).map((_, i) => <option value={String(i + 1)} key={i}>{String(i + 1) + "月"}</option>)
          }
        </select>
      </div>
    </div>
  )
})

export default TermSelect;