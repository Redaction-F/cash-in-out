import { RefObject, useState } from "react";
import Displays from "./Displays";
import TabsWrapper from "./TabsWrapper";

function MainDisplay() {
  const tab_names: string[] = ["main", "data", "edit", "set"];
  const [currentTab, setCurrentTab] = useState<string>("main");
  const display_contents: {[key: string]: RefObject<HTMLDivElement>} = {};

  function setDisplayContents(display_contents_arg: {[key: string]: RefObject<HTMLDivElement>}) {
    Object.entries(display_contents_arg).forEach(([key, value]: [string, RefObject<HTMLDivElement>]) => {
      display_contents[key] = value;
    });
  };

  function changeDisplayWrapper(tab_name: string) {
    return () => {
      display_contents[currentTab].current?.classList.remove("display-show");
      display_contents[tab_name].current?.classList.add("display-show");
      setCurrentTab(tab_name);
    }
  };

  return (
    <>
      <Displays tab_names={tab_names} setDisplayContents={setDisplayContents}/>
      <TabsWrapper changeDisplayWrapper={changeDisplayWrapper} />
    </>
  )
}

export default MainDisplay;