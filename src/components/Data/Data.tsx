import { invoke } from "@tauri-apps/api/tauri";
import { useState, useEffect } from "react";
import { CashRecord, SpecialFunctions } from "../../logic";
import Table from "./Table";
import OptionButtons from "./OptionButtons";

function Data(props: {specialFunctions: SpecialFunctions}) {
  const [tableRows, setTableRows] = useState<CashRecord[]>([]);
  const [checkedRows, setCheckedRows] = useState<[boolean, number][]>([]);
  const [checkedCount, setCheckedCount] = useState<number>(0);
  const [firstCheckedId, setFirstCheckedId] = useState<number>(NaN);

  function setCheckedRowsWrap(value: [boolean, number][]) {
    setCheckedRows(value)
  };

  useEffect(() => {
    invoke<CashRecord[]>("first_get_from_db").then(setTableRows);
  }, []);

  useEffect(() => {
    invoke<[number, (number | null)]>("count_true", {vec: checkedRows}).then((v) => {
      setCheckedCount(v[0]);
      setFirstCheckedId(v[1] === null ? NaN : v[1]);
    });
  }, [checkedRows])

  return (
    <>
      <OptionButtons checkedCount={checkedCount} firstCheckedId={firstCheckedId} specialFunctions={props.specialFunctions}/>
      <Table tableRows={tableRows} setCheckedRowsWrap={setCheckedRowsWrap}/>
    </>
  )
}

export default Data;