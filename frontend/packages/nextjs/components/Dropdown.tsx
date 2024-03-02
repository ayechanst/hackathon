import React from "react";
import Button from "./Button";

interface DropdownProps {
  dropdownListItems: string[];
  dropdownName: string;
}

const Dropdown: React.FC<DropdownProps> = ({ dropdownListItems, dropdownName }) => {
  const listItemArray = dropdownListItems;
  return (
    <>
      <div className="dropdown dropdown-bottom dropdown-end">
        <div tabIndex={0} role="button" className="btn m-3 bg-yellow-500">
          {dropdownName}
        </div>
        <ul
          tabIndex={0}
          className="dropdown-content z-[1] menu shadow bg-gradient-to-b from-yellow-500 rounded-box w-auto"
        >
          {listItemArray.map(listItem => (
            <li className="">
              <a>
                <Button key={listItem} buttonName={listItem} />
              </a>
            </li>
          ))}
        </ul>
      </div>
    </>
  );
};

export default Dropdown;
