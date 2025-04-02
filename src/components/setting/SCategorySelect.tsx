import { useRef, useState } from "react";
import { MCategorySelectFunctions, SelectMCategory, SelectSCategory, SCategorySelectFunctions } from "./logic";

function SCategorySelect(props: {
  mCategoryFunctions: MCategorySelectFunctions, 
  sCategoryFunctions: SCategorySelectFunctions, 
  additionalOption: JSX.Element[], 
  disabled: boolean, 
  defaultValue: string
}) {
  // ドロップダウンから選択中のサブカテゴリを取得
  function get(): SelectSCategory | string {
    return SelectSCategory.fromString(select.current!.value);
  }
  // サブカテゴリを読み込み再レンダリング
  function reload(mainCategory: SelectMCategory | string) {
    if (mainCategory instanceof SelectMCategory) {
      SelectSCategory.reload(mainCategory);
      setRender((prev) => 1 - prev);
    } else {
      SelectSCategory.clear();
      setRender((prev) => 1 - prev);
    }
  }
  // 選択中のMCategoryに合わせて選択できるサブカテゴリを変更
  function update() {
    reload(props.mCategoryFunctions.get!())
  }

  // select要素
  const select = useRef<HTMLSelectElement>(null);
  // 全体の再レンダリング
  // useState: ドロップダウンの再レンダリング
  const [render, setRender] = useState<number>(0);

  props.sCategoryFunctions.get = get;
  props.sCategoryFunctions.update = update;

  return (
    <select 
      id="add-category-main" 
      className="setting-select" 
      disabled={props.disabled} 
      defaultValue={props.defaultValue} 
      key={render} 
      ref={select}
    >
      <option value={SelectSCategory.none.value}>--</option>
      {
        SelectSCategory.subCategorys.map((v) => <option value={v} key={v}>{v}</option>)
      }
      {
        SelectSCategory.subCategorys.length === 0
        ? <></>
        : props.additionalOption
      }
    </select>
  )
}

export default SCategorySelect;