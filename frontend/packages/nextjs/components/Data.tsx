import { useState } from "react";
import Menu from "./Menu";
import Tabs from "./Tabs";

// const Data = () => {
//   const [tabNames, setTabNames] = useState<string[]>([
//     "NFT",
//     "NFT Holders",
//     "NFT Collections",
//     "Tokens",
//     "Token Holders",
//   ]);

//   return (
//     <>
//       <Tabs tabName={tabNames} />
//     </>
//   );
// };

// export default Data;
const Data = () => {
  const [subgraphQueryName, setSubgraphQueryName] = useState<string>("");
  function handleButtonNameFromMenu(buttonName: string) {
    setSubgraphQueryName(buttonName);
  }

  return (
    <>
      <div className="flex">
        <div className="mt-2 mx-5 w-1/5">
          <Menu sendButtonNameToMenu={handleButtonNameFromMenu} />
        </div>
        <div className="mr-5 w-4/5">
          <Tabs subgraphQuery={subgraphQueryName} />
        </div>
      </div>
    </>
  );
};

export default Data;
