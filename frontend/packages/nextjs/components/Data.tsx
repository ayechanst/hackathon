import { useState } from "react";
import Tabs from "./Tabs";

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
    </>
  );
};

export default Data;
