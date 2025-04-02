import { useEffect, useRef, useState } from "react";
import { MCategorySelectFunctions, SelectMCategory, SCategorySelectFunctions } from "./logic";

function MCategorySelect(props: {
  mCategoryFunctions: MCategorySelectFunctions, 
  sCategoryFunctions: SCategorySelectFunctions, 
  additionalOption: JSX.Element[], 
  disabled: boolean, 
  defaultValue: string, 
}) {
  // ドロップダウンから選択中のメインカテゴリを取得
  function get(): SelectMCategory | string {
    return SelectMCategory.fromString(select.current!.value);
  }
  // メインカテゴリを読み込み再レンダリング
  async function reload() {
    await SelectMCategory.reload();
    setRender((prev) => 1 - prev);
  }

  // select要素
  const select = useRef<HTMLSelectElement>(null);
  // 全体の再レンダリング
  // useState: ドロップダウンの再レンダリング
  const [render, setRender] = useState<number>(0)

  props.mCategoryFunctions.get = get;
  props.mCategoryFunctions.reload = reload;

  useEffect(() => {
    props.sCategoryFunctions.update!();
  }, [render])

  return (
    <select 
      id="add-category-main" 
      className="setting-select" 
      onChange={() => props.sCategoryFunctions.update!()} 
      disabled={props.disabled} 
      defaultValue={props.defaultValue} 
      key={render} 
      ref={select}
    >
      <option value={SelectMCategory.none.value}>--</option>
      {
        SelectMCategory.mainCategorys.map((v) => <option value={v} key={v}>{v}</option>)
      }
      {
        props.additionalOption
      }
    </select>
  )
}

export default MCategorySelect;