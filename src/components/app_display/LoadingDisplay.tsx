import { useRef } from "react";
import { LoadingDisplayFunctions } from "./logic";

function LoadingDisplay(props: {
  loadingDisplayFunction: LoadingDisplayFunctions
}) {
  function startLoading() {
    loading.current?.classList.remove("loading-loaded");
  }
  function endLoading() {
    loading.current?.classList.add("loading-loaded");
  }

  const loading = useRef<HTMLDivElement>(null);

  props.loadingDisplayFunction.start = startLoading;
  props.loadingDisplayFunction.end = endLoading;

  return (
    <div className="loading-container loading-loaded" ref={loading}>
      <div className="loading-spinner"></div>
    </div>
  )
}

export default LoadingDisplay;