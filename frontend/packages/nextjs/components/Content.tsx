import React, { useEffect, useState } from "react";
import Button from "./Button";
import Chart from "./Chart";
import SearchInput from "./SearchInput";
import { gql, useQuery } from "@apollo/client";
import { Query, queryHelper } from "~~/queryHelpers";

const Content = () => {
  const [selectedTab, setSelectedTab] = useState("Tokens");
  const [filter, setFilter] = useState("");
  const [timeFilter, setTimeFilter] = useState("");
  const [query, setQuery] = useState<Query>(queryHelper("Tokens"));
  const [searchInputValue, setSearchInputValue] = useState<string | void>("");
  const active = "text-primary";

  const { loading, error, data } = useQuery(
    gql`
      ${query.subgraphQuery}
    `,
    {
      variables: query.variables,
    },
  );

  // useEffect(() => {
  //   setQuery(queryHelper(selectedTab, filter, timeFilter));
  // }, [selectedTab, filter, timeFilter]);

  return (
    <>
      <div className="flex">
        {/* ------------------------------- start of Menu --------------------------------------- */}
        <div className="flex flex-col w-1/4">
          <div className="items-center m-10">
            <div className="flex justify-center pt-3 font-bold text-lg">Data Filtration</div>
            <div className="px-3">
              <div className="divider divider-start divider-warning">
                {selectedTab === "Tokens" ? <div>Search a Token</div> : <div>Search an NFT Collection</div>}
              </div>
              <div className="flex flex-col items-center">
                <SearchInput onFormSubmit={(value: any) => setSearchInputValue(value)} />
              </div>
            </div>
            <div className="px-3 pb-3">
              <div className="divider divider-start divider-warning">Filters Options</div>
              <div className="flex flex-col w-full">
                {selectedTab === "Tokens" ? (
                  <div>
                    <Button
                      buttonName="Newest Deployments"
                      isActive={filter === "Newest Deployments"}
                      onClick={() => setFilter("Newest Deployments")}
                    />
                    <Button buttonName="Volume" isActive={filter === "Volume"} onClick={() => setFilter("Volume")} />
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

          {/* ------------------------------------ start of TimeMenu ------------------------------ */}
          <div className="m-10 h-full bg-secondary shadow-xl rounded-lg">
            <div className="flex justify-center pt-5 font-bold text-lg">Time Line</div>
            <div className="divider"></div>
            <div className="px-3">
              <div className="flex flex-col items-center">
                <Button buttonName="1 day" isActive={timeFilter === "1 day"} onClick={() => setTimeFilter("1 day")} />
                <Button
                  buttonName="1 week"
                  isActive={timeFilter === "1 week"}
                  onClick={() => setTimeFilter("1 week")}
                />
                <Button
                  buttonName="1 month"
                  isActive={timeFilter === "1 month"}
                  onClick={() => setTimeFilter("1 month")}
                />
                <Button
                  buttonName="1 year"
                  isActive={timeFilter === "1 year"}
                  onClick={() => setTimeFilter("1 year")}
                />
                <div className="pb-3"></div>
              </div>
            </div>
          </div>
        </div>
        {/* ------------------------------------ end of TimeMenu ------------------------------ */}

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
              // checked={selectedTab === "Tokens"}
            />
            <div role="tabpanel" className="tab-content bg-base-100 border-base-300 rounded-box p-6">
              {data ? (
                <div>
                  tab1
                  <Chart data={data} />
                </div>
              ) : loading ? (
                <div> loading mothafuckin bitch</div>
              ) : (
                <div> idk bruh </div>
              )}
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
              {data ? (
                <div>
                  tab1
                  <Chart data={data} />
                </div>
              ) : loading ? (
                <div> loading mothafuckin bitch</div>
              ) : (
                <div> idk brosive</div>
              )}
            </div>
          </div>
        </div>
        {/* ---------------------------------- end of Tabs ------------------------------- */}
      </div>
    </>
  );
};
export default Content;
