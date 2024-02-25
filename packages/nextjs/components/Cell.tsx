import React from "react";

interface CellProps {
  num: number;
}
const Cell: React.FC<CellProps> = ({ num }) => {
  const isEven = num % 2 === 0;

  let cellColor = isEven ? "bg-blue-500" : "bg-red-500";
  let bgColor = "bg-gray-950";
  // let zPos = isEven ? "z-0" : "z-10";

  return (
    <>
      <div className="grid grid-cols-4 grid-rows-2 w-auto h-auto">
        <div className="row-span-2 flex flex-col flex-grow h-full">
          <div className={`${cellColor} grid grid-cols-2 h-full`}>
            {/* tll DONE*/}
            <div className={`relative ${cellColor}`}>
              <div className={`absolute rounded-tr-full inset-0 ${bgColor}`}></div>
            </div>

            {/* tlr */}
            <div className={`relative ${cellColor}`}>
              <div className="">tlr</div>
            </div>
          </div>

          {/* bottom left */}
          <div className={`${cellColor} grid grid-cols-2`}>
            {/* bottom left left */}
            <div className={`${bgColor}`}>
              <div>bll</div>
            </div>

            {/* bottom left right */}

            <div className={`relative ${bgColor}`}>
              <div className={`absolute rounded-bl-full inset-0 ${cellColor}`}></div>
            </div>
          </div>
        </div>

        <div className={`${cellColor} col-span-2 row-span-2 text-white p-4 flex`}>Data {num} wee pee pee poo</div>

        <div className="row-span-2 flex flex-col flex-grow h-full">
          {/* bottom right */}
          <div className={`${cellColor} grid grid-cols-2 h-full`}>
            {/* bottom right right */}
            <div className={`relative ${bgColor}`}>
              <div className={`absolute rounded-tr-full inset-0 ${cellColor}`}></div>
            </div>
            {/* top right right */}
            <div className={`${bgColor}`}>
              <div>trr</div>
            </div>
          </div>
          {/* bottom right */}
          <div className={`${cellColor} grid grid-cols-2`}>
            {/* bottom right left */}
            <div className="">brl</div>
            {/* bottom right right */}
            <div className={`relative ${cellColor}`}>
              <div className={`absolute rounded-bl-full inset-0 ${bgColor}`}></div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default Cell;
