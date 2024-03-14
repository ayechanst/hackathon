import React, { useEffect, useState } from "react";
import Button from "./Button";
import Chart from "./Chart";
import SearchInput from "./SearchInput";
import { gql, useQuery } from "@apollo/client";
import { Query, queryHelper } from "~~/queryHelpers";

const Content = () => {
  const [selectedTab, setSelectedTab] = useState("Tokens");
  const [filter, setFilter] = useState("volume");
  const [timeFilter, setTimeFilter] = useState("all");
  const [queryState, setQuery] = useState<any>(queryHelper("", "", "", ""));
  const [searchInputValue, setSearchInputValue] = useState<string | void>("");
  const active = "text-primary";

  const { loading, error, data } = useQuery(
    gql`
      ${queryState.subgraphQuery}
    `,
    {
      variables: queryState.variables,
    },
  );

  console.log("data: ", data);

  useEffect(() => {
    console.log("selectedTab", selectedTab);
    console.log("filter", filter);
    console.log("timeFilter", timeFilter);
    setQuery(queryHelper(selectedTab, filter, timeFilter, searchInputValue));
  }, [selectedTab, filter, timeFilter, searchInputValue]);

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
                <SearchInput
                  onFormSubmit={(value: string) => {
                    setFilter("");
                    setTimeFilter("");
                    if (value === "") {
                      setFilter("volume");
                      return;
                    } else if (value.startsWith("0x")) {
                      //remove 0x from the string
                      value = value.slice(2);
                    }
                    setSearchInputValue(value);
                  }}
                />
              </div>
            </div>
            <div className="px-3 pb-3">
              <div className="divider divider-start divider-warning">Filters Options</div>
              <div className="flex flex-col w-full">
                {selectedTab === "Tokens" ? (
                  <div>
                    <Button
                      buttonName="Newest Deployments"
                      isActive={filter === "newest"}
                      onClick={() => {
                        setTimeFilter("");
                        setFilter("newest");
                      }}
                    />
                    <Button
                      buttonName="Oldest Deployments"
                      isActive={filter === "oldest"}
                      onClick={() => {
                        setTimeFilter("");
                        setFilter("oldest");
                      }}
                    />
                    <Button buttonName="Tx Count" isActive={filter === "volume"} onClick={() => setFilter("volume")} />
                  </div>
                ) : (
                  <div>
                    <Button
                      buttonName="Oldest Deployments"
                      isActive={filter === "oldest"}
                      onClick={() => {
                        setTimeFilter("");
                        setFilter("oldest");
                      }}
                    />
                    <Button
                      buttonName="Newest Deployments"
                      isActive={filter === "newest"}
                      onClick={() => {
                        setTimeFilter("");
                        setFilter("newest");
                      }}
                    />
                    <Button buttonName="Tx Count" isActive={filter === "volume"} onClick={() => setFilter("volume")} />
                  </div>
                )}
              </div>
            </div>
          </div>
          {/* ------------------------------------ end of Menu ------------------------------ */}

          {/* ------------------------------------ start of TimeMenu ------------------------------ */}
          {filter == "volume" ? (
            <div className="m-5 h-full bg-secondary shadow-xl rounded-lg">
              <div className="flex justify-center pt-5 font-bold text-lg">Time Line</div>
              <div className="divider"></div>
              <div className="px-3">
                <div className="flex flex-col items-center">
                  <Button buttonName="1 day" isActive={timeFilter === "day"} onClick={() => setTimeFilter("day")} />
                  <Button buttonName="1 week" isActive={timeFilter === "week"} onClick={() => setTimeFilter("week")} />
                  <Button
                    buttonName="1 month"
                    isActive={timeFilter === "month"}
                    onClick={() => setTimeFilter("month")}
                  />
                  <Button buttonName="All Time" isActive={timeFilter === "all"} onClick={() => setTimeFilter("all")} />
                  <div className="pb-3"></div>
                </div>
              </div>
            </div>
          ) : (
            ""
          )}
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
              // onChange={() => setSelectedTab("Tokens")}
              onClick={() => {
                setSearchInputValue("");
                setFilter("volume");
                setTimeFilter("all");
                setSelectedTab("Tokens");
              }}
              aria-label="Tokens"
              checked={selectedTab === "Tokens"}
            />
            <div role="tabpanel" className="tab-content bg-base-100 border-base-300 rounded-box p-6">
              {data ? (
                <div>
                  <Chart data={data} />
                </div>
              ) : loading ? (
                <LoadingSpinner />
              ) : (
                <div> idk bruh </div>
              )}
            </div>
            <input
              type="radio"
              name="my_tabs_2"
              role="tab"
              className={`tab text-primary  ${selectedTab === "NFTs" ? active : "text-yellow-100"}`}
              onChange={() => {
                setSearchInputValue("");
                setFilter("volume");
                setTimeFilter("all");
                setSelectedTab("NFTs");
              }}
              aria-label="NFTs"
            />
            <div role="tabpanel" className="tab-content bg-base-100 border-base-300 rounded-box p-6">
              {data ? (
                <div>
                  <Chart data={data} />
                </div>
              ) : loading ? (
                <LoadingSpinner />
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

export const LoadingSpinner = () => {
  return (
    <div className="flex justify-center items-center">
      <div className="animate-spin rounded-full h-12 w-12 border-t-4 border-r-4 border-l-4 border-b-yellow-500 border-b-4 border-gray-900 "></div>
    </div>
  );
};

export default Content;
