import React, { useEffect, useState } from "react";
import Cell from "./Cell";

const Chart: React.FC<any> = ({ data }) => {
  return (
    <>
      <div className="flex bg-secondary p-3 rounded-2xl">
        {data ? (
          Object.keys(data[0]).map((dataKey: any, keyIndex: any) => {
            return (
              <div key={dataKey.toString()}>
                <div className="flex justify-center text-lg">{dataKey}</div>
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
      </div>
    </>
  );
};

export default Chart;
