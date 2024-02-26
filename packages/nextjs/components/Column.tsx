import React from "react";
import Cell from "./Cell";
import { animateValue, motion } from "framer-motion";

interface ColumnProps {
  rows: number;
  cols: number;
  pattern: number;
  id: number;
}

const Column: React.FC<ColumnProps> = ({ rows, cols, pattern, id }) => {
  const rowArray: number[] = [];
  const colArray: number[] = [];
  let animateValue;
  let initialValue;
  let delayValue;
  for (let i = 0; i < rows; i++) {
    rowArray.push(i);
  }
  for (let i = 0; i < cols; i++) {
    colArray.push(i);
  }

  const middleIndex = Math.floor(colArray.length / 2);
  console.log("middle index", middleIndex);

  if (id < middleIndex) {
    initialValue = -1200;
    animateValue = 0;
  } else {
    initialValue = 1200;
    animateValue = 0;
  }

  if (id === 0 || id === colArray.length - 1) {
    delayValue = 0.5;
  } else {
    delayValue = 0;
  }

  return (
    <>
      <motion.div
        initial={{ x: initialValue }}
        animate={{ x: animateValue }}
        transition={{ duration: 2, delay: delayValue }}
        className="flex flex-col"
      >
        <div>Col Header</div>
        <div>
          {rowArray.map(row => (
            <Cell key={row} num={row} pattern={pattern} />
          ))}
        </div>
      </motion.div>
    </>
  );
};

export default Column;
