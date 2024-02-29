import { useState } from "react";
import Menu from "./Menu";
import Tabs from "./Tabs";

const Data = () => {
  const [subgraphQueryName, setSubgraphQueryName] = useState<string>("erc20Transfers");
  function handleButtonNameFromMenu(buttonName: string) {
    console.log("buttonName being sent to subgraphQuery from Data.tsx: ", buttonName);
    setSubgraphQueryName(buttonName);
  }

  return (
    <>
      <div className="grid grid-cols-10">
        <div className="mt-2 mx-5 col-span-2">
          <Menu sendButtonNameToMenu={handleButtonNameFromMenu} />
        </div>
        <div className="mr-5 mt-2 col-span-8">
          <Tabs subgraphQuery={subgraphQueryName} />
        </div>
      </div>
    </>
  );
};

export default Data;
