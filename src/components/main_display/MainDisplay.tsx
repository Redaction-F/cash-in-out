import { useState, useRef } from "react";
import Displays from "./Displays";
import TabsWrapper from "./TabsWrapper";
import { DisplayHandler, DisplayName, SpecialFunctions } from "../../logic";

function MainDisplay() {
  const [currentTab, setCurrentTab] = useState<DisplayName>("main");
  const displayHandlers: {[key in DisplayName]: DisplayHandler} = {
    "main": {
    content: useRef<HTMLDivElement>(null),
    tab: useRef<HTMLInputElement>(null), 
    onClose: async() => {
      return true
    },
    onOpen: () => {},
  }, 
    "data": {
    content: useRef<HTMLDivElement>(null),
    tab: useRef<HTMLInputElement>(null), 
    onClose: async() => {
      return true
    },
    onOpen: () => {},
  }, 
    "edit": {
    content: useRef<HTMLDivElement>(null),
    tab: useRef<HTMLInputElement>(null), 
    onClose: async() => {
      return true
    },
    onOpen: () => {},
  }, 
    "set": {
    content: useRef<HTMLDivElement>(null),
    tab: useRef<HTMLInputElement>(null), 
    onClose: async() => {
      return true
    },
    onOpen: () => {},
  }};

  const specialFunctions: SpecialFunctions = {
    changeDisplay: changeDisplay, 
    startEdit: undefined
  };

  async function changeDisplay(tabName: DisplayName) {
    if (displayHandlers[currentTab].tab.current !== null && displayHandlers[tabName].tab.current !== null) {
      displayHandlers[tabName].tab.current.checked = false;
      displayHandlers[currentTab].tab.current.checked = true;
    }
    if (await displayHandlers[currentTab].onClose()) {
      if (displayHandlers[currentTab].tab.current !== null && displayHandlers[tabName].tab.current !== null) {
        displayHandlers[currentTab].tab.current.checked = false;
        displayHandlers[tabName].tab.current.checked = true;
      }
      displayHandlers[currentTab].content.current?.classList.remove("display-show");
      displayHandlers[tabName].content.current?.classList.add("display-show");
      displayHandlers[tabName].onOpen();
      setCurrentTab(tabName);
    };
  };

  return (
    <>
      <Displays displayHandlers={displayHandlers} specialFunctions={specialFunctions} />
      <TabsWrapper displayHandlers={displayHandlers} changeDisplay={changeDisplay} />
    </>
  )
}

export default MainDisplay;