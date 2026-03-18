import { RefObject } from "react";
import { DisplayName } from "./components/app_display/logic";
import { invoke } from "@tauri-apps/api/core";

// .tsxファイルで用いる型の定義
// 具体的な機能はrustで書く

// 全体共有用の関数群
export type SpecialFunctions = {
  // display切り替え
  changeDisplay: ((displayName: DisplayName) => Promise<boolean>) | undefined, 
  // edit displayで編集を開始
  startEdit: ((id: number | null) => Promise<void>) | undefined, 
  startCreate: (() => void) | undefined
}

// displayの操作用
export type DisplayHandler = {
  // displayであるdivタグの要素
  content: RefObject<HTMLDivElement | null>, 
  // displayを操作するtab
  tab: RefObject<HTMLInputElement | null>, 
  // このdisplayから遷移するときの処理
  // 返り値は遷移可能か否か
  onClose: () => Promise<boolean>, 
  // このdisplayに遷移にするときの処理
  onOpen: () => Promise<void>
}

// 出入金1単位の項目
export const cashIORecordFields = ["id", "date", "mainCategory", "subCategory", "title", "amount", "memo"] as const;
export type CashIORecordField = typeof cashIORecordFields[number];
const cashIORecordFieldSet: Set<string> = new Set(cashIORecordFields);
// InputKind判定
export function isCashIORecordField(arg: string): arg is CashIORecordField {
  return cashIORecordFieldSet.has(arg);
}
// 出入金1単位
export class CashIORecord {
  private _id: number;
  private _date: string;
  private _mainCategory: string;
  private _subCategory: string;
  private _title: string;
  private _amount: number;
  private _memo: string;

  constructor(
    id: string, 
    date: string, 
    mainCategory: MCategory | string, 
    subCategory: SCategory | string, 
    title: string, 
    amount: string, 
    memo: string
  ) {
    this._id = Number(id);
    this._date = date;
    this._mainCategory = (mainCategory instanceof MCategory) ? mainCategory.value : mainCategory;
    this._subCategory = (subCategory instanceof SCategory) ? subCategory.value : subCategory;
    this._title = title;
    this._amount = Number(amount);
    this._memo = memo;
  }

  hasProperId() {
    return !Number.isNaN(this._id);
  }

  hasProperDate() {
    return !Number.isNaN(new Date(this._date).getTime());
  }

  hasProperCategory() {
    return this.hasProperMCategory() && this.hasProperSCategory();
  }

  hasProperMCategory() {
    return !(new MCategory(this._mainCategory)).isNone();
  }

  hasProperSCategory() {
    return !(new SCategory(this._subCategory).isNone());
  }

  hasProperAmount() {
    return !Number.isNaN(this._amount);
  }

  async create() {
    if (!this.hasProperDate()) {
      throw new Error("日付が不正です。");
    } else if (!this.hasProperCategory()) {
      throw new Error("カテゴリが不正です。");
    } else if (!this.hasProperAmount()) {
      throw new Error("金額が不正です。");
    } else {
      await invoke<void>("create_record", {newRecord: this});
    }
  }

  async update() {
    if (!this.hasProperId()) {
      throw new Error("idが不正です。");
    } else if (!this.hasProperDate()) {
      throw new Error("日付が不正です。");
    } else if (!this.hasProperCategory()) {
      throw new Error("カテゴリが不正です。");
    } else if (!this.hasProperAmount()) {
      throw new Error("金額が不正です。");
    } else {
      await invoke<void>("update_record", {changedRecord: this});
    }
  }

  static async getById(id: number): Promise<CashIORecord | null> {
    return await invoke<CashIORecord | null>("get_record_by_id", {id: id});
  }

  // 今月のデータを取得
  static async getInThisMonth(): Promise<CashIORecord[]> {
    let today: Date = new Date();
    return await invoke<CashIORecord[]>("get_records_by_month", {year: today.getFullYear(), month: today.getMonth() + 1});
  }
 
  // データを取得
  static async getByMonth(year: number, month: number): Promise<CashIORecord[]> {
    if (month === null) {
      return [];
    } else {
      return await invoke<CashIORecord[]>("get_records_by_month", {year: year, month: month});
    }
  }

  static async deleteById(id: number) {
    await invoke<void>("delete_record_by_id", {id: id});
  }

  get id(): number {
    return this._id;
  };

  get date(): string {
    return String(this._date);
  };

  get mainCategory(): string {
    return this._mainCategory;
  };

  get subCategory(): string {
    return this._subCategory;
  };

  get title(): string {
    return this._title;
  };

  get amount(): number {
    return this._amount;
  };

  get memo(): string {
    return this._memo;
  };
}

// メインカテゴリ
export class MCategory {
  static allCategorys: Map<string, string[]> = new Map();
  static mainCategorys: string[] = [];
  private static _none: string = "--none";
  private _value: string;

  constructor(value: string) {
    if (MCategory.allCategorys.has(value) || value === MCategory._none) {
      this._value = value;
    } else {
      console.log("Developer error: The value is not MainCategory.");
      this._value = MCategory._none;
    }
  }

  isNone(): boolean {
    return this.value === MCategory._none;
  }

  getSubs(): string[] {
    let subs: string[] | undefined =  MCategory.allCategorys.get(this.value);
    if (subs === undefined) {
      return [];
    } else {
      return subs;
    }
  }

  async remove() {
    await invoke<void>("remove_main_category", {mainCategoryName: this._value});
  }
  
  static fromString(value: string): MCategory | string {
    if (MCategory.allCategorys.has(value) || value === MCategory._none) {
      return new MCategory(value);
    } else {
      return value;
    }
  }

  static async add(name: string) {
    await invoke<void>("add_main_category", {newMainCategoryName: name});
  }

  static async reload() {
    let allCategorys: Map<string, string[]> = await invoke<MainCategoryWithSubs[]>("get_all_categorys").then((v) => new Map(v.map((u) => [u.name, u.subs])));
    MCategory.allCategorys = allCategorys;
    MCategory.mainCategorys = Array.from(allCategorys.keys());
  }

  get value(): string {
    return this._value;
  }

  static get none(): MCategory {
    return new MCategory(MCategory._none)
  }
}

// サブカテゴリ
export class SCategory {
  private static _subCategorysSet: Set<string> = new Set();
  private static _subCategorys: string[] = [];
  private static _superCategory: MCategory = MCategory.none;
  private static _none: string = "--none";
  static other: string = "その他";
  private _value: string;

  constructor(value: string) {
    if (SCategory._subCategorysSet.has(value) || value === SCategory._none) {
      this._value = value;
    } else {
      console.log("Developer error: The value is not SubCategory.");
      this._value = SCategory._none;
    }
  }

  isNone() {
    return this.value === SCategory._none;
  }

  async remove() {
    await invoke<void>("remove_sub_category", {subCategorName: this._value, mainCategoryName: SCategory._superCategory.value});
  }
  
  static fromString(value: string): SCategory | string {
    if (SCategory._subCategorysSet.has(value) || value === SCategory._none) {
      return new SCategory(value);
    } else {
      return value;
    }
  }

  static clear() {
    SCategory._subCategorysSet = new Set();
    SCategory._subCategorys = [];
  }

  static async reload(mainCategory: MCategory) {
    let subs_raw: string[] = mainCategory.getSubs();
    let subs: string[] = [];
    let containOther: boolean = false;
    // 「その他」が含まれていれば末尾に移動
    for (let v of subs_raw) {
      if (v === SCategory.other) {
        containOther = true;
      } else {
        subs.push(v);
      }
    }
    if (containOther) {
      subs.push(SCategory.other);
    }
    SCategory._subCategorysSet = new Set(subs);
    SCategory._subCategorys = subs;
    SCategory._superCategory = mainCategory;
  }

  static async add(name: string) {
    await invoke<void>("add_sub_category", {newSubCategoryName: name, mainCategoryName: SCategory._superCategory});
  }

  get value(): string {
    return this._value;
  }

  static get none(): SCategory {
    return new SCategory(SCategory._none)
  }

  static get subCategorys(): string[] {
    return SCategory._subCategorys;
  }

  static get superCategory(): MCategory {
    return SCategory._superCategory;
  }
}

// メインカテゴリとその子であるサブカテゴリ
type MainCategoryWithSubs = {
  name: string, 
  subs: string[]
}
