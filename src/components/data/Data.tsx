import OptionButtons from "./OptionButtons";
import TermSelect from "./TermSelect";
import Table from "./Table";
import { DisplayHandler, SpecialFunctions, CashIORecord } from "../../logic";
import { dataFunctions, OptionButtonsFunctions, TableFunctions, TermSelectFunctions } from "./logic";
import IOButtions from "./IOButtons";

// data display
// 出入金データの選択・表示
function Data(props: {
  displayHandler: DisplayHandler, 
  specialFunctions: SpecialFunctions
}) {
  // 再読み込み
  async function reload() {
    tableFunctions.set!(await CashIORecord.getInThisMonth());
    termSelectFunctions.reload!();
  }

  // Data.tsxが提供する関数群
  const dataFunctions: dataFunctions = {
    reload: reload
  }
  // Table.tsxが提供する関数群
  const tableFunctions: TableFunctions = {
    set: undefined, 
    setByMonth: undefined, 
    getCheckedId: undefined, 
  }
  // TermSelect.tsxが提供する関数群
  const termSelectFunctions: TermSelectFunctions = {
    reload: undefined
  }
  // OptionButtons.tsxが提供する関数群
  const optionButtonsFunctions: OptionButtonsFunctions = {
    clearCheckedCount: undefined, 
    incCheckedCount: undefined, 
    decCheckedCount: undefined
  }

  // このタブ選択時の処理
  props.displayHandler.onOpen = reload;

  return (
    <>
      {/* ボタン群 */}
      <OptionButtons dataFunctions={dataFunctions} tableFunctions={tableFunctions} optionButtonsFunctions={optionButtonsFunctions} specialFunctions={props.specialFunctions}/>
      {/* 期間選択ドロップダウン */}
      <TermSelect tableFunctions={tableFunctions} termSelectFunctions={termSelectFunctions}/>
      {/* 表 */}
      <Table tableFunctions={tableFunctions} optionButtonsFunctions={optionButtonsFunctions}/>
      <IOButtions />
    </>
  )
}

export default Data;