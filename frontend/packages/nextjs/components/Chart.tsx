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
                  if (dataKey === "from") {
                    if (datum[dataKey] === "0000000000000000000000000000000000000000") {
                      return <Cell key={index + keyIndex} keyType={"address"} data={"BURN"} id={index} />;
                    } else {
                      return (
                        <Cell key={index + keyIndex} keyType={"address"} data={datum[dataKey].toString()} id={index} />
                      );
                    }
                  } else if (dataKey === "to") {
                    if (datum[dataKey] === "0000000000000000000000000000000000000000") {
                      return <Cell key={index + keyIndex} keyType={"address"} data={"DEPLOY"} id={index} />;
                    } else {
                      return (
                        <Cell key={index + keyIndex} keyType={"address"} data={datum[dataKey].toString()} id={index} />
                      );
                    }
                  } else if (dataKey === "id") {
                    return (
                      <Cell key={index + keyIndex} keyType={"address"} data={datum[dataKey].toString()} id={index} />
                    );
                  } else if (dataKey === "tokenId") {
                    return (
                      <Cell key={index + keyIndex} keyType={"tokenId"} data={datum[dataKey].toString()} id={index} />
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
