import React, { useEffect, useState } from "react";
import Chart from "./Chart";
import { AnimatePresence, motion } from "framer-motion";
import { useRecoilState, useRecoilValue } from "recoil";
import { useSubgraph } from "~~/hooks/scaffold-eth/useSubgraph";
import { filterButtonsArrayState, selectedTabState, subgraphQueryState } from "~~/recoil/atoms";

const Tabs = () => {
  const [selectedTab, setSelectedTab] = useRecoilState(selectedTabState);
  const tabNames = ["NFTs", "NFT Collection", "NFT Holders", "Tokens", "Token Holders"];
  const active = "text-primary";
  // let timeQuery = subgraphTimeQuery;

  // recoil

  // const { timeData } useSubgraph({
  //   subgraphTimeQuery,
  //   queryProps: { rows: 10},
  // })
  const [filterButtons, setFilterButtons] = useRecoilState(filterButtonsArrayState);

  const subgraphQuery = useRecoilValue(subgraphQueryState);

  const { data } = useSubgraph({
    subgraphQuery,
    queryProps: { rows: 10 },
  });

  return (
    <div className="grid">
      <div role="tablist" className="tabs tabs-lifted">
        {data ? (
          tabNames.map(tab => {
            const isActive = selectedTab === tab;
            return (
              <>
                <input
                  type="radio"
                  name="tabs"
                  role="tab"
                  className={`tab text-primary ${isActive ? active : "text-yellow-100"}`}
                  aria-label={tab}
                  onClick={() => setSelectedTab(tab)}
                />
                <div role="tabpanel" className="tab-content bg-base-100 border-base-300 rounded-box p-6">
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
