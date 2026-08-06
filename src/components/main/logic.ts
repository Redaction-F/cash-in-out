export type Data = {
  category: string,
  color: string,
  amount: number
};

export type GraphInfo = {
  start: number,
  end: number,
  overHalf: boolean
};

export type GraphData = {
  data: Data,
  info: GraphInfo
}