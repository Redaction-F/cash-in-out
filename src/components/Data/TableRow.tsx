import { CashRecord } from "../../logic";

function TableRow(props: {row: CashRecord, setCheckedRow: (value: boolean) => void}) {
  return(
    <tr>
      <th scope="row">
        <input type="checkbox" id={String(props.row.id)} name="row" onChange={(e) => props.setCheckedRow(e.target.checked)}/>
      </th>
      <th>
        {props.row.id}
      </th>
      <td>
        {props.row.date}
      </td>
      <td>
        {props.row.category}
      </td>
      <td>
        {props.row.title}
      </td>
      <td>
        {props.row.amount}
      </td>
      <td title={props.row.memo}>
        {props.row.memo}
      </td>
    </tr>
  );
}

export default TableRow