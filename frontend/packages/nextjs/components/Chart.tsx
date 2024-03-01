import React, { useEffect, useState } from "react";
import Cell from "./Cell";

const Chart: React.FC<any> = ({ data }) => {
  return (
    <>
      <div className="flex bg-secondary p-3 rounded-2xl place-items-center justify-between">
        <button className="rounded-full h-20 p-1 bg-primary item-center">«</button>
        <div className="place-items-center flex">
          {data ? (
            Object.keys(data[0]).map((dataKey: any, keyIndex: any) => {
              return (
                <div key={dataKey.toString()} className="px-3">
                  <div className="flex justify-center text-lg">{dataKey}</div>

                  {data.map((datum: any, index: any) => {
                    if (dataKey === "from") {
                      if (datum[dataKey] === "0000000000000000000000000000000000000000") {
                        return <Cell key={index + keyIndex} keyType={"address"} data={"BURN"} id={index} />;
                      } else {
                        return (
                          <Cell
                            key={index + keyIndex}
                            keyType={"address"}
                            data={datum[dataKey].toString()}
                            id={index}
                          />
                        );
                      }
                    } else if (dataKey === "to") {
                      if (datum[dataKey] === "0000000000000000000000000000000000000000") {
                        return <Cell key={index + keyIndex} keyType={"address"} data={"DEPLOY"} id={index} />;
                      } else {
                        return (
                          <Cell
                            key={index + keyIndex}
                            keyType={"address"}
                            data={datum[dataKey].toString()}
                            id={index}
                          />
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
        <button className="place-self-end place-self-center rounded-full h-20 p-1 bg-primary item-center">»</button>
      </div>
    </>
  );
};

export default Chart;
