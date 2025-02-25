import { invoke } from "@tauri-apps/api/tauri";
import { useState, useEffect } from "react";
import { CashRecord, SpecialFunctions } from "../../logic";
import Table from "./Table";
import OptionButtons from "./OptionButtons";

function Data(props: {specialFunctions: SpecialFunctions}) {
  const [tableRows, setTableRows] = useState<CashRecord[]>([]);
  const [checkedRows, setCheckedRows] = useState<boolean[]>([]);
  const [checkedRowsCount, setCheckedRowsCount] = useState<number>(0);

  function setCheckedRowsWrap(value: boolean[]) {
    setCheckedRows(value)
  };

  useEffect(() => {
    invoke<CashRecord[]>("first_get_from_db").then(setTableRows);
  }, []);

  useEffect(() => {
    invoke<number>("count_true", {vec: checkedRows}).then(setCheckedRowsCount);
  }, [checkedRows])

  return (
    <>
      <OptionButtons checkedRowsCount={checkedRowsCount} specialFunctions={props.specialFunctions}/>
      <Table tableRows={tableRows} setCheckedRowsWrap={setCheckedRowsWrap}/>
    </>
  )
}

export default Data;