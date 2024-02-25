import React, { useEffect, useState } from "react";
import Cell from "./Cell";

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

  console.log(rowArray);
  console.log(colArray);

  return (
    <>
      {colArray.map(col => (
        <div key={col}>
          col: {col}
          {rowArray.map(row => (
            <Cell num={row} />
          ))}
        </div>
      ))}
    </>
  );
};

export default Chart;
