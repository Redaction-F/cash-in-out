// グラフの要素となるデータの型
export type Data = {
  category: string,
  color: string,
  amount: number
};

// グラフの描画に必要な情報の型
export type GraphData = {
  category: string,
  color: string,
  amount: number,
  partialSum: number
};