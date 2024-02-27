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
      <div className="dropdown dropdown-bottom">
        <div tabIndex={0} role="button" className="btn m-1">
          {dropdownName}
        </div>
        <ul tabIndex={0} className="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
          {listItemArray.map(listItem => (
            <li>
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
