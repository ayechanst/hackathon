import React from "react";

interface ButtonProps {
  buttonName: string;
}
const Button: React.FC<ButtonProps> = ({ buttonName }) => {
  return (
    <>
      <button className="btn m-1">{buttonName}</button>
    </>
  );
};

export default Button;
