import { atom } from "recoil";

export const textState = atom({
  key: 'textState', // unique ID (with respect to other atoms/selectors)
  default: 'hellow world', // default value (aka initial value)
});

// for Data.tsx
export const subgraphQueryState = atom({
  key: 'subgraphQueryState',
  default: 'erc20Transfers',
})

export const filterButtonsArrayState = atom({
  key: 'filterButtonsArrayState',
  default: ["transfer volume"],
})

export const subgraphTimeQueryState = atom({
  key: 'subgraphTimeQueryState',
  default: '1 day',
})

export const selectedTabState = atom({
  key: 'selectedTabState',
  default: 'NFTs',
})