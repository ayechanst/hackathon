import React, { useEffect, useState } from "react";
import Chart from "./Chart";
import { motion } from "framer-motion";
import { useSubgraph } from "~~/hooks/scaffold-eth/useSubgraph";

interface TabsProps {
  subgraphQuery: string;
}

// add subGraph props
const Tabs: React.FC<TabsProps> = ({ subgraphQuery }) => {
  // add a param that inputs the correct querry for the subgraph hook
  const { data } = useSubgraph({
    subgraphQuery,
  });

  useEffect(() => {
    console.log("data in Tabs", data);
  }, [data]);
  // try to unpack tabNames from { data }
  const tabNames = ["pee", "poo"];

  const [isVisible, setIsVisible] = useState(true);

  useEffect(() => {
    setIsVisible(!isVisible);
    // const timeout = setTimeout(() => setIsVisible(!isVisible), 500); // Adjust the duration as needed
    // return () => clearTimeout(timeout);
  }, [data]);

  return (
    <motion.div animate={{ opacity: isVisible ? 1 : 0 }}>
      <div className="flex w-500">
        <div role="tablist" className="tabs tabs-lifted">
          {data ? (
            tabNames.map(tab => {
              return (
                <>
                  <input type="radio" name="my_tabs_2" role="tab" className="tab w-full" aria-label={tab} />
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
    </motion.div>
  );
};
export default Tabs;
