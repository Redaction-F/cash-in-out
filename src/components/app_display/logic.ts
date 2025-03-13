import { useRef } from "react";
import { DisplayHandler } from "../../logic";

// displayの種類
export const displayName = ["main", "data", "edit", "set"] as const;
export type DisplayName = typeof displayName[number];
// DisplayName判定
export function isDisplayName(arg: string): arg is DisplayName {
  return displayName.some((value) => value === arg);
}
// デフォルトのHandler
export function emptyHandler(): DisplayHandler {
  return {
    content: useRef<HTMLDivElement>(null),
    tab: useRef<HTMLInputElement>(null), 
    onClose: async() => {
      return true
    },
    onOpen: () => {},
  };
}