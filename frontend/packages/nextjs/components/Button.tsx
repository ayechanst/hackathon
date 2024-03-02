import React, { useState } from "react";

import { motion } from "framer-motion";


interface ButtonProps {
  buttonName: string;
  onClick: any;
  isActive: boolean;
}


const Button: React.FC<ButtonProps> = ({ buttonName, onClick, isActive }) => {

  function handleClick() {
    onClick(buttonName);
  }

  return (

    <motion.button
      onClick={() => {
        onClick(handleClick);
      }}
      className={`m-1 p-1  rounded-lg w-full ${isActive ? "active" : ""}`}
      animate={{
        backgroundColor: isActive ? "#27272a" : "#eab308",
        color: isActive ? "#eab308" : "#27272a",
        boxShadow: isActive ? "0 0 10px 2px rgba(66, 153, 225, 0.5)" : "none",
        scale: isActive ? 0.975 : 1,
      }}
      whileHover={{
        scale: 1.025,
        boxShadow: isActive ? "0 0 10px 3px rgba(66, 153, 225, 0.5)" : "0 0 10px 1px rgba(0, 0, 0, 0.2)",
      }}
      transition={{ duration: 0.5, ease: "easeInOut" }}
    >
      {buttonName}
    </motion.button>

  );
};

export default Button;
