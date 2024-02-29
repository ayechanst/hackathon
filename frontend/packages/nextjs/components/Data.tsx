import { useState } from "react";
import Menu from "./Menu";
import Tabs from "./Tabs";
import TimeRange from "./TimeRange";

const Data = () => {
  const [subgraphQueryName, setSubgraphQueryName] = useState<string>("erc20Transfers");
  const [filterButtons, setFilterButtons] = useState<string[]>([""]);

  function handleButtonNameFromMenu(buttonName: string) {
    console.log("buttonName being sent to subgraphQuery from Data.tsx: ", buttonName);
    setSubgraphQueryName(buttonName);
  }

  function handleFiltersFromTabs(filters: string[]) {
    console.log("from data: filter button array => ", filters);
    setFilterButtons(filters);
  }

  return (
    <>
      <div className="grid grid-cols-10">
        <div className="mt-2 mx-5 col-span-2">
          <Menu sendButtonNameToData={handleButtonNameFromMenu} filterButtons={filterButtons} />
        </div>
        <div className="mr-5 mt-2 col-span-7">
          <Tabs subgraphQuery={subgraphQueryName} sendFiltersToData={handleFiltersFromTabs} />
        </div>

        <div className="mr-5 mt-2 col-span-1">
          <TimeRange />
        </div>
      </div>
    </>
  );
};

export default Data;
