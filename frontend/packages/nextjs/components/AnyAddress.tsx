"use client";

import { useState } from "react";
import Link from "next/link";
import { CopyToClipboard } from "react-copy-to-clipboard";
import { Address as AddressType, isAddress } from "viem";
import { CheckCircleIcon, DocumentDuplicateIcon } from "@heroicons/react/24/outline";

type AddressProps = {
  address?: AddressType;
};
// TODO: if from address is 0000 burn, if to, mint
// TODO: make links to etherscan

export const AnyAddress = ({ address }: AddressProps) => {
  const [addressCopied, setAddressCopied] = useState(false);
  let displayAddress = address?.slice(0, 5) + "..." + address?.slice(-4);

  return (
    <div className="flex items-center">
      {displayAddress}
      {addressCopied ? (
        <CheckCircleIcon
          className="ml-1.5 text-xl font-normal text-lime-500 h-5 w-5 cursor-pointer"
          aria-hidden="true"
        />
      ) : (
        <CheckCircleIcon
          className="ml-1.5 text-xl font-normal text-sky-600 h-5 w-5 cursor-pointer"
          aria-hidden="true"
        />
      )}
      <CopyToClipboard
        text={address?.toString()}
        onCopy={() => {
          setAddressCopied(true);
          setTimeout(() => {
            setAddressCopied(false);
          }, 800);
        }}
      >
        <DocumentDuplicateIcon
          className="ml-1.5 text-xl font-normal text-sky-600 h-5 w-5 cursor-pointer"
          aria-hidden="true"
        />
      </CopyToClipboard>
    </div>
  );
};
