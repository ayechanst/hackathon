import React, { useEffect, useState } from "react";
import Chart from "./Chart";
import { useSubgraph } from "~~/hooks/scaffold-eth/useSubgraph";

interface TabsProps {
  tabName: string[];
}

const Tabs: React.FC<TabsProps> = ({ tabName }) => {
  // add a param that inputs the correct querry for the subgraph hook
  const { data } = useSubgraph({
    rows: 10,
    orderDirection: "desc",
  });

  return (
    <>
      <div className="flex w-full">
        <div role="tablist" className="tabs tabs-lifted">
          {data ? (
            tabName.map(tab => {
              return (
                <>
                  <input type="radio" name="my_tabs_2" role="tab" className="tab" aria-label={tab} />
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
    </>
  );
};
export default Tabs;
