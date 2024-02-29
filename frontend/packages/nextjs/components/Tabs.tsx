import React, { useEffect, useState } from "react";
import Chart from "./Chart";
import { motion } from "framer-motion";
import { useSubgraph } from "~~/hooks/scaffold-eth/useSubgraph";

interface TabsProps {
  subgraphQuery: string;
}

const Tabs: React.FC<TabsProps> = ({ subgraphQuery }) => {
  const [clickedTab, setClickedTab] = useState("");
  const active = "text-primary";
  const { data } = useSubgraph({
    subgraphQuery,
    queryProps: { rows: 10 },
  });

  useEffect(() => {
    console.log("data in Tabs", data);
  }, [data]);
  // try to unpack tabNames from { data }
  const tabNames = ["NFTs", "NFT Collection", "NFT Holders", "Tokens", "Token Holders"];

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
                  className={`tab w-full text-primary text-lg ${isActive ? active : "text-yellow-100"}`}
                  // className="tab"
                  aria-label={tab}
                  onClick={() => setClickedTab(tab)}
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
