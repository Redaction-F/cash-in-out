import { useRef } from "react";
import { DisplayHandler } from "../../logic";

// displayの種類
export const displayNames = ["main", "data", "edit", "setting"] as const;
export type DisplayName = typeof displayNames[number];
const displayNamesSet: Set<string> = new Set(displayNames);
// DisplayName判定
export function isDisplayName(arg: string): arg is DisplayName {
  return displayNamesSet.has(arg);
}

// デフォルトのHandler
export function emptyHandler(): DisplayHandler {
  return {
    content: useRef<HTMLDivElement>(null),
    tab: useRef<HTMLInputElement>(null), 
    onClose: async() => {
      return true
    },
    onOpen: async () => {},
  };
}