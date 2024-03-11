import React, { useEffect, useState } from "react";
import Chart from "./Chart";
import SearchInput from "./SearchInput";
import { useSubgraph } from "~~/hooks/scaffold-eth/useSubgraph";

const Tabs = () => {
  const [selectedTab, setSelectedTab] = useState("");
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
            <div className="flex flex-col items-center">
              {/* {filterButtons.map((filter: string) => {
              if (filter === "input component") {
                return <SearchInput key={filter} />;
              } else {
                return (
                  <Button
                    key={filter}
                    onClick={() => {
                      setSubgraphFilterQuery(filter);
                      setActiveButton(filter);
                    }}
                    isActive={activeButton === filter}
                    buttonName={filter}
                  />
                );
              }
            })} */}
            </div>
          </div>
        </div>
        {/* tabs */}
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
      </div>
    </>
  );
};
export default Tabs;
