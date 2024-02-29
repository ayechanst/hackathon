import React, { useEffect, useState } from "react";
import Chart from "./Chart";
import { AnimatePresence, motion } from "framer-motion";
import { useSubgraph } from "~~/hooks/scaffold-eth/useSubgraph";

interface TabsProps {
  subgraphQuery: string;
  sendFiltersToData: any;
}

// TODO: send more filters to data depending which tab its on
const Tabs: React.FC<TabsProps> = ({ subgraphQuery, sendFiltersToData }) => {
  const [clickedTab, setClickedTab] = useState("");
  const tabNames = ["NFTs", "NFT Collection", "NFT Holders", "Tokens", "Token Holders"];
  const active = "text-primary";

  const { data } = useSubgraph({
    subgraphQuery,
    queryProps: { rows: 10 },
  });

  useEffect(() => {
    console.log("data in Tabs", data);
  }, [data]);

  function handleClick(tabName: string) {
    setClickedTab(tabName);
    // move the whole component out of here
    let filters: string[];
    // everything here can also be filtered by time
    if (clickedTab === "NFTs") {
      filters = ["transfer volume"];
      sendFiltersToData(filters);
    } else if (clickedTab === "NFT Collection") {
      // will be sorted by time
      filters = ["transfer volume"];
      sendFiltersToData(filters);
    } else if (clickedTab === "NFT Holders") {
      // need to specify tokeken collection by UI input, maybe name or symbol, but easiest was will be address
      // by how much of the colletion they own
      filters = ["input component"];
      sendFiltersToData(filters);
    } else if (clickedTab === "Tokens") {
      // will be sorted by time
      filters = ["transfer volume", "relative transfer volume"];
      sendFiltersToData(filters);
    } else if (clickedTab === "Token Holders") {
      filters = ["input component", "token balance", "transfer volume"]; // token balance || what percentage of the tokens they hold
      sendFiltersToData(filters);
    } else {
      console.log("error from tabs and filters");
    }
  }
  return (
    <div className="flex w-full">
      <div role="tablist" className="tabs tabs-lifted">
        {data ? (
          tabNames.map(tab => {
            const isActive = clickedTab === tab;
            return (
              <>
                <input
                  type="radio"
                  name="tabs"
                  role="tab"
                  className={`tab text-primary ${isActive ? active : "text-yellow-100"}`}
                  aria-label={tab}
                  onClick={() => handleClick(tab)}
                />
                <div role="tabpanel" className="tab-content bg-base-100 border-base-300 w-full rounded-box p-6 ">
                  <Chart data={data} />
                </div>
              </>
            );
          })
        ) : (
          <div>no data from tabs</div>
        )}
      </div>
    </div>
  );
};

export default Tabs;

// TODO: add time input for
// TODO
