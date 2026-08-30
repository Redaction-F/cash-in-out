import PieChartContent from "./PieChartContent";
import { Data, GraphData } from "./logic";

export const margin: number = 40;
export const outerR: number = 100;
export const innerR: number = 60;
export const center: number = margin + outerR;

// 円グラフ
function PieChart(props: {
  datas: Data[]
}) {
  // 金額の総和
  const sumAmount: number = props.datas.reduce((pre, v) => pre + v.amount, 0);
  // 部分和
  let partialSum: number = 0;
  // 受け取ったdatasに部分和と総和の情報を付加
  const graphDatas: GraphData[] = props.datas.map((v) => {
    partialSum += v.amount;
    return {
      ...v,
      partialSum: partialSum - v.amount
    }
  });

  return (
    <div className="graph">
      <svg 
        className="graph-svg"
        style={{
          height: `${2 * center}px`,
          width: `${2 * center}px`
        }}
      >
        {
          graphDatas.map((v) => (<PieChartContent data={v} sumAmount={sumAmount} key={v.category} />))
        }
      </svg>
      {/* <div 
        className="graph-innertext"
        style={{
          left: `${center - 60}px`,
          top: `${center - 60}px`
        }}
      ></div> */}
    </div>
  );
}

export default PieChart;