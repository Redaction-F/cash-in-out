import { JSX, useRef, useState } from "react";
import { MCategory, SCategory } from "../../logic";
import { MCategorySelectFunctions, SCategorySelectFunctions } from "./logic";

function SCategorySelect(props: {
  mCategoryFunctions: MCategorySelectFunctions, 
  sCategoryFunctions: SCategorySelectFunctions, 
  additionalOption: JSX.Element[], 
  disabled: boolean, 
  defaultValue: string
}) {
  // ドロップダウンから選択中のサブカテゴリを取得
  function get(): SCategory | string {
    return SCategory.fromString(select.current!.value);
  }
  // サブカテゴリを読み込み再レンダリング
  function reload(mainCategory: MCategory | string) {
    if (mainCategory instanceof MCategory) {
      SCategory.reload(mainCategory);
      setRender((prev) => 1 - prev);
    } else {
      SCategory.clear();
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
    <div className="all-select-wapper">
      <select 
        id="add-category-main"
        disabled={props.disabled} 
        defaultValue={props.defaultValue} 
        key={render} 
        ref={select}
      >
        <option value={SCategory.none.value}>--</option>
        {
          SCategory.subCategorys.map((v) => <option value={v} key={v}>{v}</option>)
        }
        {
          SCategory.subCategorys.length === 0
          ? <></>
          : props.additionalOption
        }
      </select>
    </div>
  )
}

export default SCategorySelect;