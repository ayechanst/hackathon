import { useState } from "react";
import Button from "./Button";

interface TimeMenuProps {
  sendTimeButtonNameToData: any;
}

const TimeMenu: React.FC<TimeMenuProps> = ({ sendTimeButtonNameToData }) => {
  const [activeButton, setActiveButton] = useState<string | null>(null);

  const sendButtonNameToMenuOnClick = (buttonName: string) => {
    setActiveButton(buttonName);
    sendTimeButtonNameToData(buttonName);
  };

  const timeButtonArray = [
    "1 hour",
    "12 hours",
    "1 day",
    "7 days",
    "2 weeks",
    "1 month",
    "1 year",
    "2 years",
    "3 years",
    "GENESIS",
  ];

  return (
    <>
      <div className="mr-5 w-full bg-secondary shadow-xl rounded-lg">
        <div className="flex justify-center pt-5 font-bold text-lg">Time Line</div>
        <div className="divider"></div>
        <div className="px-3">
          <div className="flex flex-col items-center">
            {timeButtonArray.map((time: string) => {
              return (
                <Button
                  key={time}
                  onClick={() => sendButtonNameToMenuOnClick(time)}
                  isActive={activeButton === time}
                  buttonName={time}
                />
              );
            })}
            <div className="pb-3"></div>
          </div>
        </div>
      </div>
    </>
  );
};
export default TimeMenu;
