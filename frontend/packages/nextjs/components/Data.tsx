import { useState } from "react";
import Chart from "./Chart";
import Tabs from "./Tabs";
import { useSubgraph } from "~~/hooks/scaffold-eth/useSubgraph";

const Data = () => {
  const [tabNames, setTabNames] = useState<string[]>([
    "NFT",
    "NFT Holders",
    "NFT Collections",
    "Tokens",
    "Token Holders",
  ]);

  return (
    <>
      <Tabs tabName={tabNames} />
      {/* <Chart data={data} tabNames={tabNames} /> */}
    </>
  );
};

export default Data;
