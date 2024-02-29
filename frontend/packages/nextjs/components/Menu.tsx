import React, { useState } from "react";
import Button from "./Button";
import SearchInput from "./SearchInput";
import { motion } from "framer-motion";

interface MenuProps {
  sendButtonNameToData: any;
  filterButtons: any;
}

const Menu: React.FC<MenuProps> = ({ sendButtonNameToData, filterButtons }) => {
  const [address, setAddress] = useState("");
  const [activeButton, setActiveButton] = useState<string | null>(null);

  const sendButtonNameToMenuOnClick = (buttonName: string) => {
    setActiveButton(buttonName);
    sendButtonNameToData(buttonName);
  };

  return (
    <>
      <div className="w-full items-center">
        <div className="flex justify-center pt-3 font-bold text-lg">Data Filtration</div>
        <div className="px-3">
          <div className="divider divider-start divider-warning">Investigate</div>
          <div className="flex flex-col items-center">
            <SearchInput />
          </div>
        </div>
        <div className="px-3">
          <div className="divider divider-start divider-warning">NFTs</div>
          <div className="flex flex-col items-center">
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
          <div className="divider divider-start divider-warning">Dynamic Filters</div>
          <div className="flex flex-col items-center">
            {filterButtons.map((filter: string) => {
              if (filter === "input component") {
                return <SearchInput key={filter} />;
              } else {
                return (
                  <Button
                    key={filter}
                    onClick={() => sendButtonNameToMenuOnClick(filter)}
                    isActive={activeButton === filter}
                    buttonName={filter}
                  />
                );
              }
            })}
          </div>
        </div>
      </div>
    </>
  );
};

export default Menu;
