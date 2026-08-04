import { useEffect, useRef, useState } from "react";
import { CashIORecord } from "../../logic";
import { CheckedStates, OptionButtonsFunctions, SelectMonth, SelectYear, TableFunctions } from "./logic";

// data displayの出入金データ表
function Table(props: {
  tableFunctions: TableFunctions, 
  optionButtonsFunctions: OptionButtonsFunctions
}) {
  // 表の更新
  function set(newTableRows: CashIORecord[], sum: number) {
    table.current = newTableRows;
    tableSum.current = sum;
    setRenderTable((prev) => 1 - prev);
  }
  // 月を指定してデータベースからデータを読み込む
  async function setByMonth(year: SelectYear, month: SelectMonth) {
    if (month === null) {
      return;
    }
    set(
      await CashIORecord.getByMonth(year.value, month),
      await CashIORecord.sumByMonth(year.value, month)
    );
  }
  // 各行のチェックボックスのonChangeでeventからとれるようにラップ
  function updateCheckedRowWrap(index: number, event: React.ChangeEvent<HTMLInputElement>) {
    checkedStates.current.update(index, event.target.checked, props.optionButtonsFunctions)
  }

  // 出入金データ全体
  const table = useRef<CashIORecord[]>([]);
  // データがチェックされているか否かとid
  const checkedStates = useRef<CheckedStates>(new CheckedStates());
  const tableSum = useRef<number>(0);
  // tableRowsの更新時に更新
  // useState: Table.tsxの表の再レンダリング
  const [renderTable, setRenderTable] = useState<number>(0);

  // tableFunctionsの初期化
  props.tableFunctions.set = set;
  props.tableFunctions.setByMonth = setByMonth;
  props.tableFunctions.getCheckedId = () => checkedStates.current.getCheckedId();
  // tableRowsの更新時に実行
  useEffect(() => {
    checkedStates.current.init(table.current, props.optionButtonsFunctions);
  }, [renderTable])

  return(
    <div className="one-month-table-wrapper">
      <table className="one-month-table">
        {/* 先頭行 */}
        <thead>
          <tr>
            <th scope="col"></th>
            <th scope="col">
              ID
            </th>
            <th scope="col">
              <div className="one-month-table-header">
                <div>
                  日付
                </div>
                {/* <button className="one-month-table-filter-button"></button> */}
              </div>
            </th>
            <th scope="col">
              カテゴリ
            </th>
            <th scope="col">
              件名
            </th>
            <th scope="col">
              金額
            </th>
            <th scope="col">
              備考
            </th>
          </tr>
        </thead>
        <tbody key={renderTable}>
          {
            table.current.map((tableRow, index) => 
              // 一行
              <tr key={tableRow.id}>
                <th scope="row">
                  <input type="checkbox" id={String(tableRow.id)} name="row" onChange={updateCheckedRowWrap.bind(window, index)}/>
                </th>
                <th>
                  {tableRow.id}
                </th>
                <td>
                  {tableRow.date}
                </td>
                <td title={tableRow.mainCategory + "/" + tableRow.subCategory}>
                  {tableRow.mainCategory + "/" + tableRow.subCategory}
                </td>
                <td>
                  {tableRow.title}
                </td>
                <td>
                  {tableRow.amount}
                </td>
                <td title={tableRow.memo}>
                  {tableRow.memo}
                </td>
              </tr>
            )
          }
          <tr>
            <th scope="row"></th>
            <th></th>
            <td></td>
            <td></td>
            <td>
              合計
            </td>
            <td>
              {table.current.map((v) => v.amount).reduce((state, v) => state + v, 0)}
            </td>
            <td></td>
          </tr>
        </tbody>
      </table>
    </div>
  );
}

export default Table