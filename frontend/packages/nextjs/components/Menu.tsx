import React, { useEffect, useState } from "react";
import Button from "./Button";
import SearchInput from "./SearchInput";
import { useRecoilState, useRecoilValue } from "recoil";
import { queryState, selectedTabState } from "~~/recoil/atoms";
import { filterButtonsState } from "~~/recoil/selectors";
import { queryHelper } from "~~/utils/scaffold-eth/queryHelper";

const Menu = () => {
  const [address, setAddress] = useState("");
  const [activeButton, setActiveButton] = useState<string | null>(null);
  const [queryJuice, setQueryJuice] = useRecoilState(queryState);
  const selectedTab = useRecoilValue(selectedTabState);
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

        <div className="px-3 pb-3">
          <div className="divider divider-start divider-warning">Filters Options</div>
          <div className="flex flex-col items-center">
            {filterButtons.map((filter: string) => {
              if (filter === "input component") {
                return <SearchInput key={filter} />;
              } else {
                return (
                  <Button
                    key={filter}
                    onClick={() => {
                      const query = queryHelper(selectedTab, filter);
                      setActiveButton(filter);
                      setQueryJuice(query);
                    }}
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
