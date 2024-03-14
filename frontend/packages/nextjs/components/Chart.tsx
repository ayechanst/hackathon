"use client";

import React, { useEffect, useState } from "react";
import Cell from "./Cell";
import { motion, useAnimation } from "framer-motion";

interface ChartProps {
  data: any;
}

const Chart: React.FC<ChartProps> = ({ data }) => {
  const [pageNum, setPageNum] = useState<number>(0);
  const [paginatedData, setPaginatedData] = useState<any>(null);
  const [isAnimating, setIsAnimating] = useState(false);
  const controls = useAnimation();

  useEffect(() => {
    const dataChunk = [];
    const key = Object.keys(data);
    console.log("key");
    const newData = data[key[0]];
    for (let i = 0; i < newData.length; i += 10) {
      dataChunk.push(newData.slice(i, (i += 10)));
    }
    setPaginatedData(dataChunk);
  }, [data]);

  function handlePageChange(pageNumChange: number) {
    if (pageNum >= 0 || pageNum < paginatedData.length + 1) {
      setPageNum(prev => (prev += pageNumChange));
    }
  }

  return (
    <>
      <div className="flex bg-secondary p-3 rounded-2xl place-items-center justify-between">
        {/* <motion.button
          onClick={() => handlePageChange(-1)}
          className="rounded-full p-1 bg-primary justify-items-end flex"
        >
          <div>«</div>
          <div>{pageNum + 1}</div>
        </motion.button> */}
        {/* <motion.div className="place-items-center flex" animate={controls} transition={{ duration: 0.5 }}> */}
        <motion.div className="place-items-center flex">
          {paginatedData ? (
            // Object.keys(dividedSubgraphData[0]).map((dataKey: any, keyIndex: any) => {
            Object.keys(paginatedData[0][0]).map((dataKey: any, keyIndex: any) => {
              if (dataKey === "__typename") {
                return null;
              }
              return (
                <div key={dataKey.toString()} className="px-2">
                  <div className="flex justify-center text-lg">{dataKey}</div>

                  {/* {dividedSubgraphData.map((datum: any, index: any) => { */}
                  {paginatedData[pageNum].map((datum: any, index: any) => {
                    if (dataKey === "__typename") {
                      return null;
                    }
                    if (dataKey === "totalSupply" && datum["decimals"] !== null && datum["decimals"] !== 0) {
                      return (
                        <Cell
                          key={index + keyIndex}
                          keyType={"number"}
                          data={formatLargeNumber(datum[dataKey] / 10 ** datum["decimals"])}
                          id={index}
                        />
                      );
                    }

                    if (
                      dataKey == "count" ||
                      dataKey == "dayCount" ||
                      dataKey == "weekCount" ||
                      dataKey == "monthCount"
                    ) {
                      return (
                        <Cell
                          key={index + keyIndex}
                          keyType={"number"}
                          data={parseInt(datum[dataKey]).toLocaleString("en-US")}
                          id={index}
                        />
                      );
                    }

                    if (
                      dataKey == "volume" ||
                      dataKey == "dayVolume" ||
                      dataKey == "weekVolume" ||
                      dataKey == "monthVolume"
                    ) {
                      if (datum["decimals"] !== null && datum["decimals"] !== 0 && datum["decimals"] !== undefined) {
                        return (
                          <Cell
                            key={index + keyIndex}
                            keyType={"number"}
                            data={formatLargeNumber(datum[dataKey] / 10 ** datum["decimals"])}
                            id={index}
                          />
                        );
                      }
                    }
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
                            data={"0x" + datum[dataKey].toString()}
                            id={index}
                          />
                        );
                      }
                    } else if (dataKey === "id") {
                      return (
                        <Cell
                          key={index + keyIndex}
                          keyType={"address"}
                          data={"0x" + datum[dataKey]?.toString()}
                          id={index}
                        />
                      );
                    } else if (dataKey === "tokenId") {
                      return (
                        <Cell key={index + keyIndex} keyType={"tokenId"} data={datum[dataKey]?.toString()} id={index} />
                      );
                    } else {
                      return (
                        <Cell key={index + keyIndex} keyType={"number"} data={datum[dataKey]?.toString()} id={index} />
                      );
                    }
                  })}
                </div>
              );
            })
          ) : (
            <div>no data</div>
          )}
        </motion.div>
        {/* <button
          onClick={() => handlePageChange(1)}
          className="place-self-end rounded-full p-1 bg-primary item-center flex"
        >
          <div>{pageNum + 2}</div>
          <div>»</div>
        </button> */}
      </div>
    </>
  );
};

function formatLargeNumber(num: number, decimals = 2) {
  if (num < 1000) {
    return num.toLocaleString("en-US");
  } else if (num < 1000000) {
    return (num / 1000).toFixed(decimals) + "k";
  } else if (num < 1000000000) {
    return (num / 1000000).toFixed(decimals) + "m";
  } else if (num < 1000000000000) {
    return (num / 1000000000).toFixed(decimals) + "b";
  } else if (num < 1000000000000000) {
    return (num / 1000000000000).toFixed(decimals) + "t";
  } else if (num < 1000000000000000000) {
    return (num / 1000000000000000).toFixed(decimals) + "q";
  } else if (num < 1000000000000000000000) {
    return (num / 1000000000000000000).toFixed(decimals) + "Q";
  } else {
    return "a lot";
  }
}

export default Chart;
