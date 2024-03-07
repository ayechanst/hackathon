import React, { useEffect, useState } from "react";
import Chart from "./Chart";
import { gql, useQuery } from "@apollo/client";
import { useRecoilState } from "recoil";
import { selectedTabState } from "~~/recoil/atoms";
import { queryState } from "~~/recoil/atoms";
import { queryHelper } from "~~/utils/scaffold-eth/queryHelper";

const Tabs = () => {
  const [selectedTab, setSelectedTab] = useRecoilState(selectedTabState);
  const [nftsActive, setNftsActive] = useState(false);
  const [tokensActive, setTokensActive] = useState(true);
  const [queryJuice, setQueryJuice] = useRecoilState(queryState);

  const { loading, error, data } = useQuery(
    gql`
      ${queryJuice.subgraphQuery}
    `,
    {
      variables: queryJuice.variables,
      pollInterval: 1000,
    },
  );

  useEffect(() => {
    console.log("data tabs", data);
    console.log("loading tabs", loading);
  }, [loading, data]);

  const handleTabChange = (tab: string) => {
    if (tab === selectedTab) return;
    setSelectedTab(tab);
    setNftsActive(!nftsActive);
    setTokensActive(!tokensActive);
    const query = queryHelper(tab);
    setQueryJuice(query);
  };

  return (
    <div className="grid">
      <div role="tablist" className="tabs tabs-lifted">
        <input
          type="radio"
          name="tabs"
          role="tab"
          className={`tab tab-lifted ${tokensActive ? "tab-active" : "text-yellow-100"}`}
          aria-label="Tokens"
          onClick={() => handleTabChange("Tokens")}
        />
        <div role="tabpanel" className="tab-content bg-base-100 p-4 rounded-box">
          {loading ? <div>Loading...</div> : <Chart data={data} />}
        </div>

        <input
          type="radio"
          name="tabs"
          role="tab"
          className={`tab tab-lifted ${nftsActive ? "tab-active" : "text-yellow-100"}`}
          aria-label="NFTs"
          onClick={() => handleTabChange("NFTs")}
        />
        <div role="tabpanel" className="tab-content bg-base-100 p-4 rounded-box">
          {loading ? <div>Loading...</div> : <Chart data={data} />}
        </div>
      </div>
    </div>
  );
};

export default Tabs;
