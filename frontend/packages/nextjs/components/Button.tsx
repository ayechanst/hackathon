import React, { useState } from "react";

interface ButtonProps {
  buttonName: string;
}
const Button: React.FC<ButtonProps> = ({ buttonName }) => {
  const [active, setActive] = useState(false);
  function handleClick() {
    setActive(true);
    // render new <Tabs /> with appropriate tab names
  }
  return (
    <>
      <button onClick={handleClick} className={`m-1 p-1 bg-accent rounded-lg w-full text-primary shadow-xl`}>
        {buttonName}
      </button>
    </>
  );
};

export default Button;
