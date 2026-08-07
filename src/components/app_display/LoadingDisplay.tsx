import { forwardRef, useImperativeHandle, useRef } from "react";
import { LoadingDisplayRef } from "./logic";

const LoadingDisplay = forwardRef((_, ref: React.ForwardedRef<LoadingDisplayRef>) => {
  // ローディング画面を表示
  const startLoading = () => {
    loading.current?.classList.remove("loading-loaded");
  };
  // ローディング画面を非表示
  const endLoading = () => {
    loading.current?.classList.add("loading-loaded");
  };

  // ローディングのHTML要素
  const loading = useRef<HTMLDivElement>(null);

  // ローディング画面が提供する関数を初期化
  useImperativeHandle(ref, () => ({
    start: startLoading, 
    end: endLoading
  }));

  return (
    <div className="loading-container loading-loaded" ref={loading}>
      <div className="loading-spinner"></div>
    </div>
  )
});

export default LoadingDisplay;