import React from "react";

interface CellProps {
  data: number | string;
}

const Cell: React.FC<CellProps> = ({ data }) => {
  // const isEven = num % 2 === 0;
  let cellColor: string;
  let bgColor = "bg-indigo-900";
  cellColor = "bg-red-500";

  // if (pattern === 1) {
  //   cellColor = isEven ? "bg-red-500" : "bg-blue-500";
  // } else {
  //   cellColor = isEven ? "bg-blue-500" : "bg-red-500";
  // }

  return (
    <>
      <div className={`${cellColor} m-2 p-2`}>{data}</div>
    </>
  );

  // return (
  //   <>
  //     <div className="grid grid-cols-4 grid-rows-2 w-auto h-auto m-1">
  //       {/* Left side, separates top from bottom */}
  //       <div className="row-span-2 flex flex-col flex-grow h-full">
  //         {/* Top */}
  //         <div className={`${cellColor} relative z-1 grid grid-cols-2 h-full`}>
  //           {/* Left */}
  //           <div className={`relative ${cellColor}`}>
  //             <div className={`absolute rounded-tr-full inset-0 ${bgColor}`}></div>
  //           </div>
  //           {/* Right */}
  //           <div className={`relative ${cellColor}`}>
  //             <div className="text-transparent">tlr</div>
  //           </div>
  //         </div>

  //         {/* Bottom */}
  //         <div className={`${cellColor} relative z-0 grid grid-cols-2`}>
  //           {/* Left */}
  //           <div className={`${bgColor}`}>
  //             <div className="text-transparent">bll</div>
  //           </div>
  //           {/* Right */}
  //           <div className={`relative ${bgColor}`}>
  //             <div className={`absolute rounded-bl-full inset-0 ${cellColor}`}></div>
  //           </div>
  //         </div>
  //       </div>

  //       <div className={`${cellColor} col-span-2 row-span-2 text-white p-4 flex`}>data: {data}</div>

  //       {/* Right side */}
  //       <div className="row-span-2 flex flex-col flex-grow h-full">
  //         {/* Top */}
  //         <div className={`${cellColor} relative z-0 grid grid-cols-2 h-full`}>
  //           {/* Left */}
  //           <div className={`relative ${bgColor}`}>
  //             <div className={`absolute rounded-tr-full inset-0 ${cellColor}`}></div>
  //           </div>
  //           {/* Right */}
  //           <div className={`${bgColor}`}>
  //             <div className="absolute border border-black text-transparent">trr</div>
  //           </div>
  //         </div>
  //         {/* Bottom */}
  //         <div className={`${cellColor} relative z-1 grid grid-cols-2`}>
  //           {/* Left */}
  //           <div className="text-transparent">brl</div>
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
