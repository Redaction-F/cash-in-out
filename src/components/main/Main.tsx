import { useEffect, useRef, useState } from "react";
import { selectMonth, SelectMonth, SelectYear, TermSelectRef } from "../data/logic";
import TermSelect from "../data/TermSelect";
import PieChart from "./PieChart";
import { Data } from "./logic";
import { CashIORecord, DisplayHandler } from "../../logic";

// main display
// グラフの表示など
function Main(props: {
  displayHandler: DisplayHandler
}) {
  // 指定された月のメインカテゴリごとの金額の総和を取得
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
  };
  const setDatasFromDB = async (year: SelectYear, month: SelectMonth) => {
    setDatas(await getSumByMonthGroupByMainCategory(year, month));
  };
  const reload = async () => {
    const today: Date = new Date();
    await setDatasFromDB(new SelectYear(today.getFullYear()), selectMonth(today.getMonth() + 1));
  };

  // 円グラフに表示するデータ
  const [datas, setDatas] = useState<Data[]>([]);
  const termSelectFunctions = useRef<TermSelectRef>(null);

  props.displayHandler.onOpen = reload;
  
  return (
    <div>
      <TermSelect onTermChanged={setDatasFromDB} ref={termSelectFunctions}/>
      <PieChart datas={datas}/>
    </div>
  );
}

export default Main;