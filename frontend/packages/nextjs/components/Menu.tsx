import React, { useEffect, useState } from "react";
import Button from "./Button";
import SearchInput from "./SearchInput";
import { motion } from "framer-motion";
import { useRecoilState, useRecoilValue } from "recoil";
import { subgraphQueryState } from "~~/recoil/atoms";
import { filterButtonsState } from "~~/recoil/selectors";

const Menu = () => {
  const [address, setAddress] = useState("");
  const [activeButton, setActiveButton] = useState<string | null>(null);
  // atom
  const [subgraphQuery, setSubgraphQuery] = useRecoilState(subgraphQueryState);
  const filterButtons = useRecoilValue(filterButtonsState);

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
              onClick={() => setSubgraphQuery("erc721Transfers")}
              isActive={activeButton === "erc721Transfers"}
              buttonName="erc721Transfers"
            />
            <Button
              onClick={() => setSubgraphQuery("erc721Deployments")}
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
                    onClick={() => setSubgraphQuery(filter)}
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
