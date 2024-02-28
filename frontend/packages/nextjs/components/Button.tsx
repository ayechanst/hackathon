import React from "react";

interface ButtonProps {
  buttonName: string;
}
const Button: React.FC<ButtonProps> = ({ buttonName }) => {
  return (
    <>
      <button className="m-1 p-1 bg-accent rounded-lg w-full text-primary">{buttonName}</button>
    </>
  );
};

export default Button;
