import React from "react";
import { AnyAddress } from "./AnyAddress";
import TokenId from "./TokenId";
import { motion } from "framer-motion";

interface CellProps {
  data: number | string;
  id: number;
  keyType: string;
}

const Cell: React.FC<CellProps> = ({ data, id, keyType }) => {
  const isEven = id % 2 === 0;

  let initialValue;
  initialValue = isEven ? -1200 : 1200;

  let delayValue: number = 0;
  for (let i = 0; i <= id; i++) {
    delayValue ? (delayValue += 0.05) : (delayValue += 0.05);
  }

  return (
    // <motion.div initial={{ x: initialValue }} animate={{ x: 0 }} transition={{ duration: 0.4, delay: delayValue }}>
    <>
      <div className="shrink">
        {keyType === "address" ? (
          <div className={`border-b border-accent m-4 text-yellow-100`}>
            <AnyAddress address={data.toString()} />
          </div>
        ) : keyType === "tokenId" ? (
          <div className={`border-b border-accent m-4 text-yellow-100`}>
            <TokenId tokenId={data.toString()} />
          </div>
        ) : (
          <div className={`border-b border-accent m-4 text-yellow-100`}>{data}</div>
        )}
      </div>
    </>
    // </motion.div>
  );
};

export default Cell;
