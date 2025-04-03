import OptionButtons from "./OptionButtons";
import TermSelect from "./TermSelect";
import Table from "./Table";
import { DisplayHandler, SpecialFunctions } from "../../logic";
import { dataFunctions, getCashIORecordInThisMonth, OptionButtonsFunctions, TableFunctions, TermSelectFunctions } from "./logic";

// data display
// 出入金データの選択・表示
function Data(props: {
  displayHandler: DisplayHandler, 
  specialFunctions: SpecialFunctions
}) {
  // 再読み込み
  async function reload() {
    tableFunctions.set!(await getCashIORecordInThisMonth());
    termSelectFunctions.reload!();
  }

  // Data.tsxが提供する関数群
  const dataFunctions: dataFunctions = {
    init: reload
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
      <OptionButtons dataFunctions={dataFunctions} tableFunctions={tableFunctions} optionButtonsFunctions={optionButtonsFunctions} specialFunctions={props.specialFunctions}/>
      <TermSelect tableFunctions={tableFunctions} termSelectFunctions={termSelectFunctions}/>
      <Table tableFunctions={tableFunctions} optionButtonsFunctions={optionButtonsFunctions}/>
    </>
  )
}

export default Data;