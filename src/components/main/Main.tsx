import { useState } from "react";
import { SelectMonth, SelectYear, TermSelectFunctions } from "../data/logic";
import TermSelect from "../data/TermSelect";
import Graph from "./Graph";
import { Data } from "./logic";
import { CashIORecord } from "../../logic";

function Main() {
  const [datas, setDatas] = useState<Data[]>([]);

  const getSumByMonthGroupByMainCategory = async (year: SelectYear, month: SelectMonth): Promise<Data[]> => {
    if (month === null) {
      return [];
    }
    const sumGroupByMainCategory = await CashIORecord.sumByMonthGroupByMainCategory(year.value, month);
    return sumGroupByMainCategory.map((v) => ({
      category: v[0],
      color: `#${(Math.floor(Math.random() * 256 * 256 * 256)).toString(16)}`,
      amount: v[1]
    }));
  }

  const termSelectFunctions: TermSelectFunctions = {
    reload: undefined
  };

  return (
    <div>
      <TermSelect onTermChanged={async (year, month) => {
        setDatas(await getSumByMonthGroupByMainCategory(year, month));
      }} termSelectFunctions={termSelectFunctions}/>
      <Graph datas={datas}/>
    </div>
  );
}

export default Main;