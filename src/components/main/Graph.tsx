import GraphContent from "./GraphContent";
import { Data, GraphData } from "./logic";

export const margin: number = 40;
export const outerR: number = 100;
export const innerR: number = 60;
export const center: number = margin + outerR;

function Graph(props: {
  datas: Data[]
}) {

  const sumAmount: number = props.datas.reduce((pre, v) => pre + v.amount, 0);
  let partialSum: number = 0;
  const graphDatas: GraphData[] = props.datas.map((v) => {
    partialSum += v.amount;
    return {
      data: v,
      info: {
        start: 2 * Math.PI * (3 / 4 + (partialSum - v.amount) / sumAmount),
        end: 2 * Math.PI * (3 / 4 + partialSum / sumAmount),
        overHalf: 2 * v.amount > sumAmount
      }
    }
  });

  return (
    <div className="graph-container">
      <svg 
        className="graph"
        style={{
          height: `${2 * center}px`,
          width: `${2 * center}px`
        }}
      >
        {
          graphDatas.map((v) => (<GraphContent data={v.data} info={v.info} key={v.data.category} />))
        }
      </svg>
      <div 
        className="graph-innertext"
        style={{
          left: `${center - 60}px`,
          top: `${center - 60}px`
        }}
      >
        円グラフ
      </div>
    </div>
  );
}

export default Graph;