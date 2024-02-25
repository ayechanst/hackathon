import React from "react";
import Cell from "./Cell";

interface ColumnProps {
  rows: number;
  pattern: number;
}

const Column: React.FC<ColumnProps> = ({ rows, pattern }) => {
  const rowArray: number[] = [];
  for (let i = 0; i < rows; i++) {
    rowArray.push(i);
  }

  return (
    <>
      <div className="flex flex-col">
        <div>Col Header</div>
        <div>
          {rowArray.map(row => (
            <Cell key={row} num={row} pattern={pattern} />
          ))}
        </div>
      </div>
    </>
  );
};

export default Column;
