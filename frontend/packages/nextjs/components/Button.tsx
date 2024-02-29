import React, { useState } from "react";

interface ButtonProps {
  buttonName: string;
  onClick: any;
  isActive: boolean;
}
const Button: React.FC<ButtonProps> = ({ buttonName, onClick, isActive }) => {
  let activeClass;
  if (isActive) {
    activeClass = "bg-zinc-800 text-accent shadow-md shadow-accent-500/50";
  } else {
    activeClass = "bg-accent text-primary";
  }
  function handleClick() {
    onClick(buttonName);
  }

  return (
    <>
      <button
        onClick={() => {
          onClick(handleClick);
        }}
        // onClick={() => buttonName}
        className={`m-1 p-1 ${activeClass} rounded-lg w-full `}
      >
        {buttonName}
      </button>
    </>
  );
};

export default Button;
