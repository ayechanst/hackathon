import React, { useEffect, useState } from "react";
import Chart from "./Chart";
import { useSubgraph } from "~~/hooks/scaffold-eth/useSubgraph";

interface TabsProps {
  tabName: string[];
}

const Tabs: React.FC<TabsProps> = ({ tabName }) => {
  const { data } = useSubgraph({
    rows: 20,
    orderDirection: "desc",
  });
  return (
    <>
      <div role="tablist" className="tabs tabs-lifted">
        {data ? (
          tabName.map((tab: any) => {
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
    </>
  );
};
export default Tabs;
