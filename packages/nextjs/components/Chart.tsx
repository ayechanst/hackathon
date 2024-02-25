import React, { useEffect, useState } from "react";
import Cell from "./Cell";
import Column from "./Column";

interface ChartProps {
  numRows: number;
  numCols: number;
}

const Chart: React.FC<ChartProps> = ({ numRows, numCols }) => {
  const rowArray: number[] = [];
  for (let i = 0; i < numRows; i++) {
    rowArray.push(i);
  }

  const colArray: number[] = [];
  for (let i = 0; i < numCols; i++) {
    colArray.push(i);
  }

  return (
    <>
      {colArray.map(col => (
        <Column key={col} rows={numRows} />
      ))}
      {/* {colArray.map(col => (
        <div key={col}>
          col: {col}
          {rowArray.map(row => (
            <Cell key={row} num={row} />
          ))}
        </div>
      ))} */}
    </>
  );
};

export default Chart;
