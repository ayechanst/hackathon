import React from "react";
import { motion } from "framer-motion";

interface CellProps {
  data: number | string;
  id: number;
  // pattern: number;
}

// add patern param if u want to make a cool pattern with the cells
const Cell: React.FC<CellProps> = ({ data, id }) => {
  const isEven = id % 2 === 0;

  let initialValue;
  initialValue = isEven ? -1200 : 1200;

  let delayValue: number = 0;
  for (let i = 0; i <= id; i++) {
    delayValue ? (delayValue += 0.05) : (delayValue += 0.05);
  }

  let cellColor: string;
  cellColor = "bg-gradient-to-r from-yellow-500 from-1% via-zinc-700 from-2% to-zinc-700";
  // if (pattern === 1) {
  //   cellColor = isEven
  //     ? "bg-gradient-to-l from-zinc-700 from-90% to-yellow-500"
  //     : "bg-gradient-to-l from-yellow-500 from-10% to-zinc-700";
  // } else {
  //   cellColor = isEven
  //     ? "bg-gradient-to-l from-yellow-500 from-90% to-zinc-700"
  //     : "bg-gradient-to-l from-zinc-700 from-10% to-yellow-500";
  // }

  // let border = isEven ? "border-b border-t border-accent" : "";

  return (
    <motion.div initial={{ x: initialValue }} animate={{ x: 0 }} transition={{ duration: 0.4, delay: delayValue }}>
      <div className={`border-b border-accent m-4 text-yellow-100`}>
        {data} id: {id}
      </div>
    </motion.div>
  );

  // return (
  //   <>
  //     <div className="grid grid-cols-4 grid-rows-2 h-auto m-1">
  //       {/* Left side, separates top from bottom */}
  //       <div className="row-span-2 grid grid-col  ">
  //         {/* Top */}
  //         <div className={`${cellColor} relative z-1 grid grid-cols-2 h-full`}>
  //           {/* Left */}
  //           <div className={`relative ${cellColor}`}>
  //             <div className={`absolute rounded-tr-full inset-0 ${bgColor}`}>tll</div>
  //           </div>
  //           {/* Right */}
  //           <div className={`relative ${cellColor}`}>
  //             <div className="absolute inset-0 ">tlr</div>
  //           </div>
  //         </div>

  //         {/* Bottom */}
  //         <div className={`${cellColor} relative z-0 grid grid-cols-2`}>
  //           {/* Left */}
  //           <div className={`${bgColor}`}>
  //             <div className="absolute inset-0 ">bll</div>
  //           </div>
  //           {/* Right */}
  //           <div className={`relative ${bgColor}`}>
  //             <div className={`absolute rounded-bl-full inset-0 ${cellColor}`}>blr</div>
  //           </div>
  //         </div>
  //       </div>

  //       <div className={`${cellColor} col-span-2 row-span-2 text-white p-4 flex`}>data: {data}</div>

  //       {/* Right side */}
  //       <div className="row-span-2 grid grid-col">
  //         {/* Top */}
  //         <div className={`${cellColor} relative z-0 grid grid-cols-2 h-full`}>
  //           {/* Left */}
  //           <div className={`relative ${bgColor} border border-black`}>
  //             <div className={`absolute rounded-tr-full inset-0   ${cellColor} `}>tlr</div>
  //           </div>
  //           {/* Right */}
  //           <div className={`${bgColor}`}>
  //             <div className="absolute border border-black inset-0 ">trr</div>
  //           </div>
  //         </div>

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
};
export default Cell;
