import { useState } from "react";
import Button from "./Button";
import { AddressInput } from "./scaffold-eth";

const Menu = () => {
  const [address, setAddress] = useState("");
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
            <Button buttonName="NFT" />
            <Button buttonName="NFT Holders" />
            <Button buttonName="NFT Collections" />
          </div>
        </div>
        <div className="px-3 pb-3">
          <div className="divider divider-start divider-warning">Tokens</div>
          <div className="flex flex-col items-start">
            <Button buttonName="Tokens" />
            <Button buttonName="Token Holders" />
          </div>
        </div>
      </div>
    </>
  );
};

export default Menu;
