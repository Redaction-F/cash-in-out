import { useState } from "react";
import { TermSelectFunctions } from "../data/logic";
import TermSelect from "../data/TermSelect";
import Graph from "./Graph";
import { Data } from "./logic";

function Main() {
  const [datas, setDatas] = useState<Data[]>([
    {
      category: "Category1",
      color: "#3f8f8f",
      amount: 2000
    },
    {
      category: "Category2",
      color: "#3faf4f",
      amount: 1000
    },
    {
      category: "Category3",
      color: "#af3f2f",
      amount: 2000
    },
  ]);

  const termSelectFunctions: TermSelectFunctions = {
    reload: undefined
  };

  return (
    <div>
      <TermSelect onTermChanged={async (year, month) => {
        
      }} termSelectFunctions={termSelectFunctions}/>
      <Graph datas={datas}/>
    </div>
  );
}

export default Main;