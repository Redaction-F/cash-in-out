import { JSX, useEffect, useRef, useState } from "react";
import { MCategory } from "../../logic";
import { MCategorySelectFunctions, SCategorySelectFunctions } from "./logic";

function MCategorySelect(props: {
  mCategoryFunctions: MCategorySelectFunctions, 
  sCategoryFunctions: SCategorySelectFunctions, 
  additionalOption: JSX.Element[], 
  disabled: boolean, 
  defaultValue: string, 
}) {
  // ドロップダウンから選択中のメインカテゴリを取得
  function get(): MCategory | string {
    return MCategory.fromString(select.current!.value);
  }
  // メインカテゴリを読み込み再レンダリング
  async function reload() {
    await MCategory.reload();
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
    <div className="all-select-wapper">
      <select 
        id="add-category-main"
        onChange={() => props.sCategoryFunctions.update!()} 
        disabled={props.disabled} 
        defaultValue={props.defaultValue} 
        key={render} 
        ref={select}
      >
        <option value={MCategory.none.value}>--</option>
        {
          MCategory.mainCategorys.map((v) => <option value={v} key={v}>{v}</option>)
        }
        {
          props.additionalOption
        }
      </select>
    </div>
  )
}

export default MCategorySelect;