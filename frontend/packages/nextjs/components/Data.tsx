import { useState } from "react";
import Menu from "./Menu";
import Tabs from "./Tabs";
import TimeMenu from "./TimeMenu";

const Data = () => {
  const [subgraphQueryName, setSubgraphQueryName] = useState<string>("erc20Transfers");
  const [filterButtons, setFilterButtons] = useState<string[]>([""]);
  const [subgraphTimeQuery, setSubgraphTimeQuery] = useState<string>("1 Hour");

  function handleButtonNameFromMenu(buttonName: string) {
    console.log("buttonName being sent to subgraphQuery from Data.tsx: ", buttonName);
    setSubgraphQueryName(buttonName);
  }

  function handleTimeButtonNameFromTimeMenu(buttonName: string) {
    setSubgraphTimeQuery(buttonName);
  }

  return (
    <>
      <div className="grid grid-cols-10">
        <div className="mt-2 mx-5 col-span-2">
          <Menu sendButtonNameToData={handleButtonNameFromMenu} filterButtons={filterButtons} />
        </div>
        <div className="mr-5 mt-2 col-span-7">
          <Tabs subgraphQuery={subgraphQueryName} setFilterButtons={setFilterButtons} />
        </div>
        <div className="mr-5 mt-2 col-span-1">
          <TimeMenu sendTimeButtonNameToData={handleTimeButtonNameFromTimeMenu} />
        </div>
      </div>
    </>
  );
};

export default Data;
