import React from "react";

interface TokenIdProps {
  tokenId: string;
}

const TokenId: React.FC<TokenIdProps> = ({ tokenId }) => {
  let displayTokenId;
  if (tokenId.length > 10) {
    displayTokenId = tokenId?.slice(0, 5) + "..." + tokenId?.slice(-4);
  } else {
    displayTokenId = tokenId;
  }
  return (
    <>
      <div>{displayTokenId}</div>
    </>
  );
};

export default TokenId;
