import React, { useEffect, useState } from "react";
import Cell from "./Cell";

const Chart: React.FC<any> = ({ data }) => {
  console.log("data from Chart: ", data);
  const keyArray = Object.keys(data[0]);
  console.log("keys from data: ", keyArray);
  return (
    <>
      <div className="flex bg-secondary p-3 rounded-2xl">
        {data ? (
          Object.keys(data[0]).map((dataKey: any, keyIndex: any) => {
            return (
              <div key={dataKey.toString()}>
                <div className="flex justify-center text-lg">{dataKey}</div>
                {data.map((datum: any, index: any) => {
                  if (dataKey === "from" || dataKey === "to") {
                    return (
                      <Cell key={index + keyIndex} keyType={"address"} data={datum[dataKey].toString()} id={index} />
                    );
                  } else {
                    return (
                      <Cell key={index + keyIndex} keyType={"number"} data={datum[dataKey].toString()} id={index} />
                    );
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
