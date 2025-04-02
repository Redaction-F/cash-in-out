import { invoke } from "@tauri-apps/api";

// Category.tsxが提供する機能群
export type CategoryFunctions = {
  reloadCategorySetting: (() => Promise<void>) | undefined
}

// メインカテゴリとその子であるサブカテゴリ
type MainCategoryWithSubs = {
  name: string, 
  subs: string[]
}
// すべてのカテゴリを取得
async function getAllCategorys(): Promise<Map<string, string[]>> {
  let allCategorys: MainCategoryWithSubs[] = await invoke<MainCategoryWithSubs[]>("get_all_categorys");
  return new Map(allCategorys.map((v) => [v.name, v.subs]));
}

// MCategorySelect.tsxが提供する関数群
export type MCategorySelectFunctions = {
  get: (() => SelectMCategory | string) | undefined, 
  reload: (() => Promise<void>) | undefined
}
// SCategorySelect.tsxが提供する関数群
export type SCategorySelectFunctions = {
  get: (() => SelectSCategory | string) | undefined, 
  update: (() => void) | undefined, 
}

// 選択できるメインカテゴリ+α
export class SelectMCategory {
  static allCategorys: Map<string, string[]> = new Map();
  static mainCategorys: string[] = [];
  private static _none: string = "--none";
  private _value: string;

  constructor(value: string) {
    if (SelectMCategory.allCategorys.has(value) || value === SelectMCategory._none) {
      this._value = value;
    } else {
      console.log("Developer error: The value is not MainCategory.");
      this._value = SelectMCategory._none;
    }
  }

  isNone(): boolean {
    return this.value === SelectMCategory._none;
  }

  getSubs(): string[] {
    let subs: string[] | undefined =  SelectMCategory.allCategorys.get(this.value);
    if (subs === undefined) {
      return [];
    } else {
      return subs;
    }
  }
  
  static fromString(value: string): SelectMCategory | string {
    if (SelectMCategory.allCategorys.has(value) || value === SelectMCategory._none) {
      return new SelectMCategory(value);
    } else {
      return value;
    }
  }

  static async reload() {
    let allCategorys: Map<string, string[]> = await getAllCategorys();
    SelectMCategory.allCategorys = allCategorys;
    SelectMCategory.mainCategorys = Array.from(allCategorys.keys());
  }

  get value(): string {
    return this._value;
  }

  static get none(): SelectMCategory {
    return new SelectMCategory(SelectMCategory._none)
  }
}

// 選択できるサブカテゴリ+α
export class SelectSCategory {
  static subCategorysSet: Set<string> = new Set();
  static subCategorys: string[] = [];
  private static _none: string = "--none";
  static other: string = "その他";
  private _value: string;

  constructor(value: string) {
    if (SelectSCategory.subCategorysSet.has(value) || value === SelectSCategory._none) {
      this._value = value;
    } else {
      console.log("Developer error: The value is not SubCategory.");
      this._value = SelectSCategory._none;
    }
  }

  isNone() {
    return this.value === SelectSCategory._none;
  }
  
  static fromString(value: string): SelectSCategory | string {
    if (SelectSCategory.subCategorysSet.has(value) || value === SelectSCategory._none) {
      return new SelectSCategory(value);
    } else {
      return value;
    }
  }

  static clear() {
    SelectSCategory.subCategorysSet = new Set();
    SelectSCategory.subCategorys = [];
  }

  static async reload(mainCategory: SelectMCategory) {
    let subs_raw: string[] = mainCategory.getSubs();
    let subs: string[] = [];
    let containOther: boolean = false;
    // 「その他」が含まれていれば末尾に移動
    for (let v of subs_raw) {
      if (v === SelectSCategory.other) {
        containOther = true;
      } else {
        subs.push(v);
      }
    }
    if (containOther) {
      subs.push(SelectSCategory.other);
    }
    SelectSCategory.subCategorysSet = new Set(subs);
    SelectSCategory.subCategorys = subs;
  }

  get value(): string {
    return this._value;
  }

  static get none(): SelectSCategory {
    return new SelectSCategory(SelectSCategory._none)
  }
}
