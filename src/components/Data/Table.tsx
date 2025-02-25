import { useEffect, useState } from "react";
import { CashRecord } from "../../logic";
import TableRow from "./TableRow";

function Table(props: {tableRows: CashRecord[], setCheckedRowsWrap: (value: boolean[]) => void}) {
  const [checkedRows, setCheckedRows] = useState<boolean[]>([]);

  function setCheckedRow(index: number, value: boolean) {
    if (checkedRows.length === 0) {
      setCheckedRows(Array(props.tableRows.length).fill(false));
    };
    setCheckedRows((prevState) => prevState.map((v, i) => i === index ? value : v));
  };

  useEffect(() => {
    props.setCheckedRowsWrap(checkedRows);
  }, [checkedRows]);

  return(
    <table className="one-month-table">
      <thead>
        <tr>
          <th scope="col"></th>
          <th scope="col">
            ID
          </th>
          <th scope="col">
            日時
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
            <TableRow row={tableRow} setCheckedRow={(value: boolean) => setCheckedRow(index, value)} key={tableRow.id}/>
          )
        }
      </tbody>
    </table>
  );
}

export default Table