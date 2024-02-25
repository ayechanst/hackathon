import React from "react";
import Cell from "./Cell";

interface ColumnProps {
  rows: number;
}

const Column: React.FC<ColumnProps> = ({ rows }) => {
  const rowArray: number[] = [];
  for (let i = 0; i < rows; i++) {
    rowArray.push(i);
  }

  return (
    <>
      <div className="flex flex-col">
        <div>Col Header</div>
        <div>
          {rowArray.map(row => {
            if (row % 2 === 0) {
              return <Cell key={row} num={row} isEvenCol={true} />;
            } else {
              return <Cell key={row} num={row} isEvenCol={false} />;
            }
          })}
        </div>
      </div>
    </>
  );
};

export default Column;
