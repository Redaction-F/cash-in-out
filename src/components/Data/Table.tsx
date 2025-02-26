import { CashRecord } from "../../logic";

// data displayの出入金データ表
function Table(props: {tableRows: CashRecord[], updateCheckedRow: (index: number, value: boolean) => void}) {
  // 各行のチェックボックスのonChangeでeventからとれるようにラップ
  function updateCheckedRowWrap(index: number, event: React.ChangeEvent<HTMLInputElement>) {
    props.updateCheckedRow(index, event.target.checked)
  }

  return(
    <table className="one-month-table">
      {/* 先頭行 */}
      <thead>
        <tr>
          <th scope="col"></th>
          <th scope="col">
            ID
          </th>
          <th scope="col">
            日付
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
      <tbody>
        {
          props.tableRows.map((tableRow, index) => 
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
              <td>
                {tableRow.category}
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
      </tbody>
    </table>
  );
}

export default Table