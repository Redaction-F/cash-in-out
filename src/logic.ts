// .tsxファイルで用いる型の定義
// 具体的な機能はrustで書く

export type OneMonthTableRow = {
  id: number, 
  date: String, 
  category: String | null, 
  title: string, 
  amount: number, 
  memo: string
};