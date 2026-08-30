import OptionButtons from "./OptionButtons";
import TermSelect from "./TermSelect";
import Table from "./Table";
import { DisplayHandler, Global, CashIORecord } from "../../logic";
import { OptionButtonsRef, TableRef, TermSelectRef } from "./logic";
import IOButtions from "./IOButtons";
import { useRef } from "react";

// data display
// 出入金データの選択・表示
function Data(props: {
  displayHandler: DisplayHandler, 
  global: Global
}) {
  // 再読み込み
  const reload = async () => {
    tableRef.current!.set(
      await CashIORecord.getInThisMonth(), 
      await CashIORecord.sumInThisMonth()
    );
    termSelectRef.current!.reload();
  };

  // Table.tsxが提供する関数群
  const tableRef = useRef<TableRef>(null);
  // TermSelect.tsxが提供する関数群
  const termSelectRef = useRef<TermSelectRef>(null);
  // OptionButtons.tsxが提供する関数群
  const optionButtonsRef = useRef<OptionButtonsRef>(null);

  // このタブ選択時の処理
  props.displayHandler.onOpen = reload;

  return (
    <>
      {/* ボタン群 */}
      <OptionButtons reload={reload} getCheckedIds={() => (tableRef.current?.getCheckedIds())} global={props.global} ref={optionButtonsRef}/>
      {/* 期間選択ドロップダウン */}
      <TermSelect onTermChanged={async (year, month) => {tableRef.current?.setByMonth(year, month)}} ref={termSelectRef}/>
      {/* 表 */}
      <Table onUpdateCheckBoxes={(checkedCount) => {optionButtonsRef.current?.onUpdateCheckBoxes(checkedCount)}} ref={tableRef}/>
      {/* 出入力のためのボタン */}
      <IOButtions />
    </>
  )
}

export default Data;