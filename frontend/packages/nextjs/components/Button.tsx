import React, { useState } from "react";

interface ButtonProps {
  buttonName: string;
  onClick: any;
}
const Button: React.FC<ButtonProps> = ({ buttonName, onClick }) => {
  const [active, setActive] = useState(false);
  let bg;
  let text;
  if (active) {
    bg = "bg-zinc-900";
    text = "text-accent";
  } else {
    bg = "bg-accent";
    text = "text-primary";
  }
  function handleClick() {
    onClick(buttonName);
    setActive(true);
    // render new <Tabs /> with appropriate tab names
  }

  return (
    <>
      <button
        onClick={() => {
          onClick(handleClick);
          setActive(!active);
        }}
        className={`m-1 p-1 ${bg} ${text} rounded-lg w-full shadow-lg`}
      >
        {buttonName}
      </button>
    </>
  );
};

export default Button;
