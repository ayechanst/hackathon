import React, { useEffect, useState } from "react";
import Button from "./Button";
import Chart from "./Chart";
import SearchInput from "./SearchInput";
import { useSubgraph } from "~~/hooks/scaffold-eth/useSubgraph";

const Content = () => {
  const [selectedTab, setSelectedTab] = useState("");
  const [filter, setFilter] = useState("");
  const [timeFilter, setTimeFilter] = useState("");
  const active = "text-primary";

  const { data } = useSubgraph({
    subgraphQuery: selectedTab,
    queryProps: { rows: 80 },
  });

  // useEffect(() => {
  //   if (!data) return;
  //   // setSelectedTab(selectedTab);
  //   setSubgraphDataArray(data);
  // }, [data]);

  return (
    <>
      <div className="flex">
        {/* ------------------------------- start of Menu --------------------------------------- */}
        <div className="w-1/4 items-center m-10">
          <div className="flex justify-center pt-3 font-bold text-lg">Data Filtration</div>
          <div className="px-3">
            <div className="divider divider-start divider-warning">Investigate</div>
            <div className="flex flex-col items-center">
              <SearchInput />
            </div>
          </div>
          <div className="px-3 pb-3">
            <div className="divider divider-start divider-warning">Filters Options</div>
            <div className="flex flex-col w-full">
              {selectedTab === "Tokens" ? (
                <div>
                  <Button buttonName="tx" isActive={filter === "tx"} onClick={() => setFilter("tx")} />
                  <Button buttonName="volume" isActive={filter === "volume"} onClick={() => setFilter("volume")} />
                </div>
              ) : (
                <div>
                  <Button buttonName="butt" isActive={filter === "butt"} onClick={() => setFilter("butt")} />
                  <Button buttonName="fee" isActive={filter === "fee"} onClick={() => setFilter("fee")} />
                </div>
              )}
            </div>
          </div>
        </div>
        {/* ------------------------------------ end of Menu ------------------------------ */}

        {/* ------------------------------------ start of Tabs ------------------------------ */}
        <div className="w-3/4 mt-10 mr-10">
          <div role="tablist" className="tabs tabs-lifted">
            <input
              type="radio"
              name="my_tabs_2"
              role="tab"
              className={`tab text-primary  ${selectedTab === "Tokens" ? active : "text-yellow-100"}`}
              onClick={() => setSelectedTab("Tokens")}
              aria-label="Tokens"
            />
            <div role="tabpanel" className="tab-content bg-base-100 border-base-300 rounded-box p-6">
              tab 1 <Chart data={data} />
            </div>
            <input
              type="radio"
              name="my_tabs_2"
              role="tab"
              className={`tab text-primary  ${selectedTab === "NFTs" ? active : "text-yellow-100"}`}
              onClick={() => setSelectedTab("NFTs")}
              aria-label="NFTs"
            />
            <div role="tabpanel" className="tab-content bg-base-100 border-base-300 rounded-box p-6">
              tab 2<Chart data={data} />
            </div>
          </div>
        </div>
        {/* ---------------------------------- end of Tabs ------------------------------- */}
      </div>
    </>
  );
};
export default Content;
