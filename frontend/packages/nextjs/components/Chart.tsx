import React, { useEffect, useState } from "react";
import Cell from "./Cell";

interface Address {
  id: string;
  numTxs: number;
  isContract: boolean;
}

interface ChartProps {
  sampleData: {
    addresses: Address[];
  };
}

const Chart: React.FC<ChartProps> = ({ sampleData }) => {
  const dataArray = sampleData.addresses;
  const sampleDatum = dataArray[0];
  const keysArray = Object.keys(sampleDatum);
  console.log(keysArray);
  const numOfCols = Object.keys(sampleDatum).length;
  // for each thing in an object, make a cell that holds just that
  // for each key, make a column
  // for each object, make a row
  return (
    <>
      {keysArray.map(key => {
        dataArray.map(datum => {
          <Cell key={datum} data={datum.} />;

        })
      })}
    </>
  );
};

export default Chart;
