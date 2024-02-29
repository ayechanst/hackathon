import React, { useState } from "react";
import Button from "./Button";
import { AddressInput } from "./scaffold-eth";

interface MenuProps {
  sendButtonNameToMenu: any;
}

const Menu: React.FC<MenuProps> = ({ sendButtonNameToMenu }) => {
  const [address, setAddress] = useState("");
  const [activeButton, setActiveButton] = useState<string | null>(null);

  const sendButtonNameToMenuOnClick = (buttonName: string) => {
    setActiveButton(buttonName);
    sendButtonNameToMenu(buttonName);
  };

  return (
    <>
      <div className="w-full bg-secondary shadow-xl rounded-lg">
        <div className="flex justify-center pt-3 font-bold text-lg">Archives</div>
        <div className="px-3">
          <div className="divider divider-start divider-warning">Investigate</div>
          <div className="flex flex-col items-start">
            {/* <input type="text" placeholder="input an address" className="w-full max-w-xs p-2 border-accent" /> */}
            <AddressInput onChange={setAddress} value={address} placeholder="Input your address" />
          </div>
        </div>
        <div className="px-3">
          <div className="divider divider-start divider-warning">NFTs</div>
          <div className="flex flex-col items-start">
            <Button
              onClick={() => sendButtonNameToMenuOnClick("erc721Transfers")}
              isActive={activeButton === "erc721Transfers"}
              buttonName="erc721Transfers"
            />
            <Button
              onClick={() => sendButtonNameToMenuOnClick("erc721Deployments")}
              isActive={activeButton === "erc721Deployments"}
              buttonName="erc721Deployments"
            />
          </div>
        </div>
        <div className="px-3 pb-3">
          <div className="divider divider-start divider-warning">Tokens</div>
          <div className="flex flex-col items-start">
            <Button
              onClick={() => sendButtonNameToMenuOnClick("erc20Transfers")}
              isActive={activeButton === "erc20Transfers"}
              buttonName="erc20Transfers"
            />
            <Button
              onClick={() => sendButtonNameToMenuOnClick("erc20Deployments")}
              isActive={activeButton === "erc20Deployments"}
              buttonName="erc20Deployments"
            />
          </div>
        </div>
      </div>
    </>
  );
};

export default Menu;