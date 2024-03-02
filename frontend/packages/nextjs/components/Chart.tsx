import React, { useEffect, useState } from "react";
import Cell from "./Cell";
import { useRecoilState, useRecoilValue } from "recoil";
import { paginatedPageNumState, subgraphDataArrayState } from "~~/recoil/atoms";
import { dividedSubgraphDataState } from "~~/recoil/selectors";

const Chart = () => {
  let [paginatedPageNum, setPaginatedPageNum] = useRecoilState(paginatedPageNumState);
  const dividedSubgraphData = useRecoilValue(dividedSubgraphDataState);
  const subgraphDataArray = useRecoilValue(subgraphDataArrayState);

  return (
    <>
      <div className="flex bg-secondary p-3 rounded-2xl place-items-center justify-between">
        <button
          onClick={() => {
            if (paginatedPageNum >= 1) {
              setPaginatedPageNum((paginatedPageNum -= 1));
            }
          }}
          className="rounded-full p-1 bg-primary justify-items-end flex"
        >
          <div>«</div>
          <div>{paginatedPageNum + 1}</div>
        </button>
        <div className="place-items-center flex">
          {dividedSubgraphData ? (
            Object.keys(dividedSubgraphData[0]).map((dataKey: any, keyIndex: any) => {
              return (
                <div key={dataKey.toString()} className="px-3">
                  <div className="flex justify-center text-lg">{dataKey}</div>

                  {dividedSubgraphData.map((datum: any, index: any) => {
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
        <div>
          <button
            onClick={() => {
              if (paginatedPageNum < subgraphDataArray.length / 10 - 1) {
                setPaginatedPageNum((paginatedPageNum += 1));
              }
            }}
            className="place-self-end place-self-center rounded-full p-1 bg-primary item-center flex"
          >
            <div>{paginatedPageNum + 2}</div>
            <div>»</div>
          </button>
        </div>
      </div>
    </>
  );
};

export default Chart;
