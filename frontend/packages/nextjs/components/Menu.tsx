const Menu = () => {
  return (
    <>
      <div className="card w-40 bg-secondary shadow-xl">
        <div className="p-3">
          <div className="divider divider-start divider-warning">NFTs</div>
          <div className="flex flex-start flex-col items-start">
            <button>NFT</button>
            <button>NFT Holders</button>
            <button>NFT Collections</button>
          </div>
        </div>
        <div className="p-3">
          <div className="divider divider-start divider-warning">Tokens</div>
          <div className="flex flex-start flex-col items-start">
            <button>Tokens</button>
            <button>Token Holders</button>
          </div>
        </div>
      </div>
    </>
  );
};

export default Menu;
