import { atom } from "recoil";

export const searchInputQueryState = atom({
  key: "searchInputQueryState", // unique ID (with respect to other atoms/selectors)
  default: "", // default value (aka initial value)
});

// this sets the props in the query hook
export const subgraphFilterQueryState = atom({
  key: "subgraphQueryState",
  default: "erc20Transfers",
});


// Chart.tsx
export const paginatedPageNumState = atom({
  key: "paginationPageNumState",
  default: 0,
});

export const subgraphQueryState = atom({
  key: "subgraphqueryState",
  default: "NFTs",
});
