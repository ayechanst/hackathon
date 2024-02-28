import Button from "./Button";

const Menu = () => {
  return (
    <>
      <div className="w-full bg-secondary shadow-xl rounded-lg">
        <div className="flex items-center justify-center">Archives</div>
        <div className="p-3 ">
          <div className="divider divider-start divider-warning">NFTs</div>
          <div className="flex flex-col items-start">
            <Button buttonName="NFT" />
            <Button buttonName="NFT Holders" />
            <Button buttonName="NFT Collections" />
          </div>
        </div>
        <div className="p-3">
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
