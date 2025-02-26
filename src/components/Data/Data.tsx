import { invoke } from "@tauri-apps/api/tauri";
import { useState, useEffect } from "react";
import Table from "./Table";
import OptionButtons from "./OptionButtons";
import { CashRecord, SpecialFunctions } from "../../logic";

// data display
// 出入金データの選択・表示
function Data(props: {specialFunctions: SpecialFunctions}) {
  // checkedRowsの更新
  function updateCheckedRows(index: number, value: boolean) {
    setCheckedRows((prevState) => prevState.map((v, i) => i === index ? [value, v[1]] : v));
  };

  // 出入金データ全体
  const [tableRows, setTableRows] = useState<CashRecord[]>([]);
  // データがチェックされているか否かとid
  const [checkedRows, setCheckedRows] = useState<[boolean, number][]>([]);
  // チェックされているデータの数
  const [checkedCount, setCheckedCount] = useState<number>(0);
  // 最初のチェックされているデータ(ない場合はnull)
  const [firstCheckedId, setFirstCheckedId] = useState<number | null>(null);

  // 初期化処理
  useEffect(() => {
    invoke<CashRecord[]>("first_get_from_db").then(setTableRows);
  }, []);

  // tableRowsの更新時に実行
  useEffect(() => {
    setCheckedRows(tableRows.map((value) => [false, value.id]));
  }, [tableRows])

  // checkedRowsの更新時に実行
  useEffect(() => {
    // checkedCount, firstCheckedIdの更新
    invoke<[number, (number | null)]>("count_and_get_first", {vec: checkedRows}).then((v) => {
      setCheckedCount(v[0]);
      setFirstCheckedId(v[1]);
    });
  }, [checkedRows])

  return (
    <>
      <OptionButtons checkedCount={checkedCount} firstCheckedId={firstCheckedId} specialFunctions={props.specialFunctions}/>
      <Table tableRows={tableRows} updateCheckedRow={updateCheckedRows}/>
    </>
  )
}

export default Data;