import React, { useEffect, useState } from "react";
import Cell from "./Cell";

// interface ChartProps {
//   data: [
//     {
//       id: string;
//       numTxs: number;
//       isContract: boolean;
//     },
//   ];
// }

const Chart: React.FC<any> = ({ data }) => {
  return (
    <>
      {data ? (
        Object.keys(data[0]).map((dataKey: any, keyIndex: any) => {
          return (
            <div key={dataKey.toString()}>
              {data.map((datum: any, index: any) => {
                if (keyIndex % 2 === 0) {
                  return <Cell key={index + keyIndex} data={datum[dataKey].toString()} id={index} pattern={0} />;
                } else {
                  return <Cell key={index + keyIndex} data={datum[dataKey].toString()} id={index} pattern={1} />;
                }
              })}
            </div>
          );
        })
      ) : (
        <div>no data</div>
      )}
    </>
  );
};

export default Chart;
