import React, { useEffect, useState } from "react";
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
      {colArray.map(col => {
        {
          if (col % 2 === 0) {
            return <Column key={col} id={col} cols={numCols} rows={numRows} pattern={0} />;
          } else {
            return <Column key={col} id={col} cols={numCols} rows={numRows} pattern={1} />;
          }
        }
      })}
    </>
  );
};

export default Chart;
