import { invoke } from "@tauri-apps/api/tauri";
import { useState, useEffect, useRef } from "react";
import OptionButtons from "./OptionButtons";
import DropDown from "./DropDown";
import Table from "./Table";
import { CashRecord, DisplayHandler, SpecialFunctions } from "../../logic";
import { CheckedState, DropDownFunction } from "./logic";

// data display
// 出入金データの選択・表示
function Data(props: {displayHandler: DisplayHandler, specialFunctions: SpecialFunctions}) {
  // 月を指定してデータベースからデータを読み込む
  function setTableRowsByMonth(year: number, month: number) {
    invoke<CashRecord[]>("get_records_by_month", {year: year, month: month}).then(setTableRows)
  }
  // checkedRowsの更新
  function updateCheckedRows(index: number, value: boolean) {
    if (checkedStates.current[index].isChecked && !value) {
      checkedStates.current[index].isChecked = false;
      setCheckedCount((prev) => prev - 1);
    } else if (!checkedStates.current[index].isChecked && value) {
      checkedStates.current[index].isChecked = true;
      setCheckedCount((prev) => prev + 1);
    };
  };
  // チェックされているもののうち最初の要素のidを取得する
  function getFirstCheckedId(): number | null {
    let checkedRows = checkedStates.current.filter((v) => v.isChecked);
    if (checkedRows.length === 0) {
      return null;
    } else {
      return checkedRows[0].id
    }
  }

  // 出入金データ全体
  // useState: Table.tsxの表の再レンダリング
  const [tableRows, setTableRows] = useState<CashRecord[]>([]);
  // データがチェックされているか否かとid
  const checkedStates = useRef<CheckedState[]>([]);
  // チェックされているデータの数
  // useState: OptionButton.tsxのボタンの再レンダリング
  const [checkedCount, setCheckedCount] = useState<number>(0);
  // 初回レンダリング判定
  const firstRender = useRef<boolean>(true);
  const dropDownFunction: DropDownFunction = {
    updateTable: undefined
  }
  // このディスプレイに遷移時の処理
  props.displayHandler.onOpen = () => dropDownFunction.updateTable!();

  // 初期化処理
  useEffect(() => {
    if (firstRender.current) {
      let today: Date = new Date();
      setTableRowsByMonth(today.getFullYear(), today.getMonth() + 1);
      firstRender.current = false;
    }
  }, []);
  // tableRowsの更新時に実行
  useEffect(() => {
    checkedStates.current = tableRows.map((v) => ({
      id: v.id, 
      isChecked: false
    }));
  }, [tableRows])

  return (
    <>
      <OptionButtons checkedCount={checkedCount} getFirstCheckedId={getFirstCheckedId} specialFunctions={props.specialFunctions}/>
      <DropDown dropDownFunction={dropDownFunction} setTableRowsFromMonth={setTableRowsByMonth} />
      <Table tableRows={tableRows} updateCheckedRow={updateCheckedRows}/>
    </>
  )
}

export default Data;