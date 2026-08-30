import { useState } from "react";
import { center, innerR, outerR as outerRDefault } from "./PieChart";
import { GraphData } from "./logic";

// 座標を表す型
type Position = {
  x: number,
  y: number
};
// Position型を作る
const position = (x: number, y: number) => {
  return {
    x,
    y
  }
}

// 円グラフの要素
function PieChartContent(props: {
  data: GraphData,
  sumAmount: number
}) {
  // ホバー時の動作
  const onHover = () => {
    // 大きくする
    setOuterR(outerRDefault + 20);
  };
  // ホバーが離れたときの操作
  const onLeave = () => {
    // 大きさを元に戻す
    setOuterR(outerRDefault);
  };

  // 外側の弧の半径の大きさ
  const [outerR, setOuterR] = useState<number>(outerRDefault);
  // 始まりの角度
  const start = 2 * Math.PI * (3 / 4 + props.data.partialSum / props.sumAmount);
  // 終わりの角度
  const end = 2 * Math.PI * (3 / 4 + (props.data.partialSum + props.data.amount) / props.sumAmount);
  // 50%より大きいか
  const overHalf = 2 * props.data.amount > props.sumAmount;
  // 外側の弧の始まりの点の座標
  const outerStart: Position = position(center + Math.round(outerR * Math.cos(start)), center + Math.round(outerR * Math.sin(start)));
  // 外側の弧の終わりの点の座標
  const outerEnd: Position = position(center + Math.round(outerR * Math.cos(end)), center + Math.round(outerR * Math.sin(end)));
  // 内側の弧の始まりの点の座標
  const innerStart: Position = position(center + Math.round(innerR * Math.cos(start)), center + Math.round(innerR * Math.sin(start)));
  // 内側の弧の終わりの点の座標
  const innerEnd: Position = position(center + Math.round(innerR * Math.cos(end)), center + Math.round(innerR * Math.sin(end)));
  // 文字列の座標
  const textPosition: Position = position(
    center + Math.round((outerR + innerR) / 2 * Math.cos((start + end) / 2)),
    center + Math.round((outerR + innerR) / 2 * Math.sin((start + end) / 2)),
  );

  return (
    <>
      <path 
        d={`M ${outerStart.x} ${outerStart.y} 
A ${outerR} ${outerR} 0 ${overHalf ? 1 : 0} 1 ${outerEnd.x} ${outerEnd.y} 
L ${innerEnd.x} ${innerEnd.y} 
A ${innerR} ${innerR} 0 ${overHalf ? 1: 0} 0 ${innerStart.x} ${innerStart.y} 
L ${outerStart.x} ${outerStart.y}`}
        fill={`${props.data.color}`} 
        onMouseEnter={() => onHover()}
        onMouseLeave={() => onLeave()}
      />
      <text x={textPosition.x} y={textPosition.y} textAnchor="middle"><tspan 
        className="graph-content-title" 
        stroke="var(--color-text)"
        strokeWidth="5px"
      >{props.data.category}</tspan></text>
      <text x={textPosition.x} y={textPosition.y} textAnchor="middle"><tspan 
        className="graph-content-title" 
        stroke="var(--color-content-1)" 
        strokeWidth="1px"
      >{props.data.category}</tspan></text>
    </>
  )
}

export default PieChartContent;