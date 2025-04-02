import OptionButtons from "./OptionButtons";
import TermSelect from "./TermSelect";
import Table from "./Table";
import { DisplayHandler, SpecialFunctions } from "../../logic";
import { getCashIORecordInThisMonth, OptionButtonsFunctions, TableFunctions, TermSelectFunctions } from "./logic";

// data display
// 出入金データの選択・表示
function Data(props: {
  displayHandler: DisplayHandler, 
  specialFunctions: SpecialFunctions
}) {
  // Table.tsxが提供する関数群
  const tableFunctions: TableFunctions = {
    setTableRows: undefined, 
    setTableRowsByMonth: undefined, 
    getFirstCheckedId: undefined, 
  }
  // TermSelect.tsxが提供する関数群
  const termSelectFunctions: TermSelectFunctions = {
    init: undefined
  }
  // OptionButtons.tsxが提供する関数群
  const optionButtonsFunctions: OptionButtonsFunctions = {
    clearCheckedCount: undefined, 
    incCheckedCount: undefined, 
    decCheckedCount: undefined
  }

  // このタブ選択時の処理
  props.displayHandler.onOpen = async () => {
    tableFunctions.setTableRows!(await getCashIORecordInThisMonth());
    termSelectFunctions.init!();
  };

  return (
    <>
      <OptionButtons tableFunctions={tableFunctions} optionButtonsFunctions={optionButtonsFunctions} specialFunctions={props.specialFunctions}/>
      <TermSelect tableFunctions={tableFunctions} termSelectFunctions={termSelectFunctions}/>
      <Table tableFunctions={tableFunctions} optionButtonsFunctions={optionButtonsFunctions}/>
    </>
  )
}

export default Data;