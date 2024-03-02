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
  // const isEven = id % 2 === 0;

  return (
    // <motion.div animate={{ x: 0 }} }}>
    <>
      <div className="">
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

//         {/* Bottom */}
//         <div className={`${cellColor} relative z-1 grid grid-cols-2`}>
//           {/* Left */}
//           <div className="  border border-black inset-0">brl</div>
//           {/* Right */}
//           <div className={`relative ${cellColor}`}>
//             <div className={`absolute rounded-bl-full inset-0 border border-black ${bgColor}`}></div>
//           </div>
//         </div>
//       </div>
//     </div>
//   </>
// );
export default Cell;
