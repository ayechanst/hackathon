import React, { useEffect, useState } from "react";
import Cell from "./Cell";
import { motion, useAnimation } from "framer-motion";
import { useRecoilState, useRecoilValue } from "recoil";
import { paginatedPageNumState, subgraphDataArrayState } from "~~/recoil/atoms";
import { dividedSubgraphDataState } from "~~/recoil/selectors";

const Chart = () => {
  let [paginatedPageNum, setPaginatedPageNum] = useRecoilState(paginatedPageNumState);
  const dividedSubgraphData = useRecoilValue(dividedSubgraphDataState);
  const subgraphDataArray = useRecoilValue(subgraphDataArrayState);
  const [isAnimating, setIsAnimating] = useState(false);
  const controls = useAnimation();

  useEffect(() => {
    if (isAnimating) {
      controls.start({ opacity: 0, x: -100 });
    } else {
      controls.start({ opacity: 1, x: 0 });
    }
  }, [isAnimating]);

  const handlePageChange = (pageNumChange: any) => {
    setPaginatedPageNum(prevPageNum => {
      let newPageNum = prevPageNum + pageNumChange;
      if (newPageNum < 0) {
        newPageNum = 0;
      } else if (newPageNum >= Math.ceil(subgraphDataArray.length / 10)) {
        newPageNum = Math.ceil(subgraphDataArray.length / 10) - 1;
      }
      setIsAnimating(true);
      setTimeout(() => {
        setIsAnimating(false);
      }, 400); // Set the timeout duration to match the animation duration
      return newPageNum;
    });
  };

  return (
    <>
      <div className="flex bg-secondary p-3 rounded-2xl place-items-center justify-between">
        <motion.button
          onClick={() => handlePageChange(-1)}
          className="rounded-full p-1 bg-primary justify-items-end flex"
        >
          <div>«</div>
          <div>{paginatedPageNum + 1}</div>
        </motion.button>
        <motion.div className="place-items-center flex" animate={controls} transition={{ duration: 0.5 }}>
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
        <button
          onClick={() => handlePageChange(1)}
          className="place-self-end place-self-center rounded-full p-1 bg-primary item-center flex"
        >
          <div>{paginatedPageNum + 2}</div>
          <div>»</div>
        </button>
      </div>
    </>
  );
};

export default Chart;
