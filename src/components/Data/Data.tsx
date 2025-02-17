import { invoke } from "@tauri-apps/api/tauri";
import { useState, useEffect } from "react";
import { OneMonthTableRow } from "../../logic";
import Table from "./Table";
import OptionButtons from "./OptionButtons";

function Data() {
  const [tableRows, setTableRows] = useState<OneMonthTableRow[]>([]);
  const [checkedRows, setCheckedRows] = useState<boolean[]>([]);
  const [checkedRowsCount, setCheckedRowsCount] = useState<number>(0);

  function setCheckedRowsWrap(value: boolean[]) {
    setCheckedRows(value)
  };

  useEffect(() => {
    invoke<OneMonthTableRow[]>("first_get_from_db", {dataTitle: "data"}).then(setTableRows);
  }, []);

  useEffect(() => {
    invoke<number>("count_true", {vec: checkedRows}).then(setCheckedRowsCount);
  }, [checkedRows])

  return (
    <>
      <OptionButtons checkedRowsCount={checkedRowsCount}/>
      <Table tableRows={tableRows} setCheckedRowsWrap={setCheckedRowsWrap}/>
    </>
  )
}

export default Data;