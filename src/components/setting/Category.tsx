import { useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { CategoryFunctions, MCategorySelectFunctions, SelectMCategory, SelectSCategory, SCategorySelectFunctions } from "./logic";
import MCategorySelect from "./MCategorySelect";
import SCategorySelect from "./SCategorySelect";

// カテゴリ関連の設定
function Category(props: {
  categoryFunction: CategoryFunctions
}) {
  // 選択しているメインカテゴリを取得
  function getSelectedMCategory(): SelectMCategory | SelectMainCategoryAdditional {
    let mainCategoryName: SelectMCategory | string = mCategorySelectFunctions.get!();
    if (mainCategoryName instanceof SelectMCategory || mainCategoryName === selectAddMainCategory) {
      return mainCategoryName;
    } else {
      console.log("Developer error: The value is not MainCategory(Setting).");
      return SelectMCategory.none;
    }
  }
  // 選択しているサブカテゴリを取得
  function getSelectedSCategory(): SelectSCategory | SelectSubCategoryAdditional {
    let subCategoryName: SelectSCategory | string = sCategorySelectFunctions.get!();
    if (subCategoryName instanceof SelectSCategory || subCategoryName === selectRemoveMainCategory || subCategoryName === selectAddSubCategory) {
      return subCategoryName;
    } else {
      console.log("Developer error: The value is not SubCategory(Setting).");
      return SelectSCategory.none;
    }
  }
  // カテゴリの追加
  async function addCategory() {
    let categoryName: string = inputedCategoryName.current!.value;
    if (categoryName === "") {
      alert("カテゴリ名を入力してください。");
      return;
    }
    let mainCategoryName: SelectMCategory | SelectMainCategoryAdditional = getSelectedMCategory();
    let subCategoryName: SelectSCategory | SelectSubCategoryAdditional = getSelectedSCategory();
    let addResult: Promise<void>;
    if (mainCategoryName instanceof SelectMCategory && mainCategoryName.isNone()) {
      alert("メインカテゴリを選択してください。");
      return;
    } else if (mainCategoryName === selectAddMainCategory) {
      addResult = invoke<void>("add_main_category", {newMainCategoryName: categoryName});
    } else {
      if (subCategoryName === selectAddSubCategory) {
        addResult = invoke<void>("add_sub_category", {newSubCategoryName: categoryName, mainCategoryName: mainCategoryName.value});
      } else {
        alert("サブカテゴリを追加する場合は、「(サブカテゴリを追加)」を選択してください。");
        return;
      }
    }
    addResult.then(async () => {
      alert("カテゴリを追加しました。");
      await reloadCategorySetting();
    }, (e) => {
      console.log(e);
      alert("エラーが発生しました。エラーメッセージは以下の通りです。\n" + String(e));
    });
  }
  // カテゴリの削除
  async function removeCategory() {
    let mainCategoryName: SelectMCategory | SelectMainCategoryAdditional = getSelectedMCategory();
    let subCategoryName: SelectSCategory | SelectSubCategoryAdditional = getSelectedSCategory();
    let removeResult: Promise<void>;
    if ((mainCategoryName instanceof SelectMCategory && mainCategoryName.isNone()) || mainCategoryName === "--addMainCategory") {
      alert("メインカテゴリを選択してください。");
      return;
    } else {
      if ((subCategoryName instanceof SelectSCategory && subCategoryName.isNone()) || subCategoryName === "--addSubCategory") {
        alert("メインカテゴリを削除する場合は、「(メインカテゴリを削除)」を選択してください。");
        return;
      } else if (subCategoryName === "--removeMainCategory") {
        removeResult = invoke<void>("remove_main_category", {mainCategoryName: mainCategoryName.value});
      } else {
        removeResult = invoke<void>("remove_sub_category", {subCategoryName: subCategoryName.value, mainCategoryName: mainCategoryName.value});
      }
    }
    removeResult.then(async () => {
      alert("カテゴリを削除しました。");
      await reloadCategorySetting();
    }, (e) => {
      console.log(e);
      alert("エラーが発生しました。エラーメッセージは以下の通りです。\n" + String(e));
    })
  }
  // カテゴリの再読み込み
  async function reloadCategorySetting() {
    await mCategorySelectFunctions.reload!();
    sCategorySelectFunctions.update!();
    setInputRender((prev) => 1 - prev);
  }

  // mCategorySelect追加選択肢
  const selectAddMainCategory = "--addMainCategory";
  const selectMainCategoryAdditional = [selectAddMainCategory] as const;
  type SelectMainCategoryAdditional = typeof selectMainCategoryAdditional[number];
  // sCategorySelect追加選択肢
  const selectRemoveMainCategory = "--removeMainCategory";
  const selectAddSubCategory = "--addSubCategory";
  const selectSubCategoryAdditional = [selectRemoveMainCategory, selectAddSubCategory] as const;
  type SelectSubCategoryAdditional = typeof selectSubCategoryAdditional[number];
  // カテゴリ名入力フォーム
  const inputedCategoryName = useRef<HTMLInputElement>(null);
  // SCategorySelect.tsxが提供する関数群
  const sCategorySelectFunctions: SCategorySelectFunctions = {
    get: undefined, 
    update: undefined
  };
  // MCategorySelect.tsxが提供する関数群
  const mCategorySelectFunctions: MCategorySelectFunctions = {
    get: undefined, 
    reload: undefined
  };
  // カテゴリ名入力フォームの初期化
  // useState: カテゴリ名入力フォームの再レンダリング
  const [inputRender, setInputRender] = useState<number>(0);

  // categoryFunctionの初期化
  props.categoryFunction.reloadCategorySetting = reloadCategorySetting;

  return (
    <div className="setting-section">
      カテゴリの追加・削除
      <hr />
      <div className="setting-row">
        <div className="setting-label">メインカテゴリ</div>
        <MCategorySelect 
          mCategoryFunctions={mCategorySelectFunctions} 
          sCategoryFunctions={sCategorySelectFunctions} 
          additionalOption={[
            <option value={selectAddMainCategory} key={selectAddMainCategory}>(メインカテゴリを追加)</option>
          ]} 
          disabled={false} 
          defaultValue={SelectMCategory.none.value}
        />
      </div>
      <div className="setting-row">
        <div className="setting-label">サブカテゴリ</div>
        <SCategorySelect 
          mCategoryFunctions={mCategorySelectFunctions} 
          sCategoryFunctions={sCategorySelectFunctions} 
          additionalOption={[
            <option value={selectRemoveMainCategory} key={selectRemoveMainCategory}>(メインカテゴリを削除)</option>, 
            <option value={selectAddSubCategory} key={selectAddSubCategory}>(サブカテゴリを追加)</option>
          ]} 
          disabled={false} 
          defaultValue={SelectSCategory.none.value}
        />
      </div>
      <div className="setting-row">
        <div className="setting-label">追加するカテゴリ名</div>
        <input className="setting-input" defaultValue="" key={inputRender} ref={inputedCategoryName} />
      </div>
      <div className="setting-row">
        <div className="setting-label"></div>
        <button className="setting-button" onClick={addCategory}>追加</button>
        <button className="setting-button" onClick={removeCategory}>削除</button>
      </div>
    </div>
  )
}

export default Category;