import { useState } from "react";
import { center, innerR, outerR as outerRDefault } from "./Graph";
import { Data, GraphInfo } from "./logic";

function GraphContent(props: {
  data: Data,
  info: GraphInfo
}) {
  const [outerR, setOuterR] = useState<number>(outerRDefault);

  const setIsHover = (isHover: boolean) => {
    if (isHover) {
      setOuterR(outerRDefault + 20);
    } else {
      setOuterR(outerRDefault);
    }
  }

  const outerStart: [number, number] = [center + Math.round(outerR * Math.cos(props.info.start)), center + Math.round(outerR * Math.sin(props.info.start))];
  const outerEnd: [number, number] = [center + Math.round(outerR * Math.cos(props.info.end)), center + Math.round(outerR * Math.sin(props.info.end))];
  const innerStart: [number, number] = [center + Math.round(innerR * Math.cos(props.info.start)), center + Math.round(innerR * Math.sin(props.info.start))];
  const innerEnd: [number, number] = [center + Math.round(innerR * Math.cos(props.info.end)), center + Math.round(innerR * Math.sin(props.info.end))];

  const textPosition: [number, number] = [
    center + Math.round((outerR + innerR) / 2 * Math.cos((props.info.start + props.info.end) / 2)),
    center + Math.round((outerR + innerR) / 2 * Math.sin((props.info.start + props.info.end) / 2)),
  ];

  return (
    <>
      <path 
        d={`M ${outerStart[0]} ${outerStart[1]} 
A ${outerR} ${outerR} 0 ${props.info.overHalf ? 1 : 0} 1 ${outerEnd[0]} ${outerEnd[1]} 
L ${innerEnd[0]} ${innerEnd[1]} 
A ${innerR} ${innerR} 0 ${props.info.overHalf ? 1: 0} 0 ${innerStart[0]} ${innerStart[1]} 
L ${outerStart[0]} ${outerStart[1]}`}
        fill={`${props.data.color}`} 
        onMouseEnter={() => setIsHover(true)}
        onMouseLeave={() => setIsHover(false)}
      />
      <text x={textPosition[0]} y={textPosition[1]} textAnchor="middle"><tspan 
        className="graph-content-title-border" 
        stroke="var(--color-text)"
        strokeWidth="5px"
      >{props.data.category}</tspan></text>
      <text x={textPosition[0]} y={textPosition[1]} textAnchor="middle"><tspan 
        className="graph-content-title" 
        stroke="var(--color-content-1)" 
        strokeWidth="1px"
      >{props.data.category}</tspan></text>
    </>
  )
}

export default GraphContent;