import { atom } from "recoil";

export const searchInputQueryState = atom({
  key: 'searchInputQueryState', // unique ID (with respect to other atoms/selectors)
  default: '', // default value (aka initial value)
});

export const subgraphFilterQueryState = atom({
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

// Tabs.tsx

export const selectedTabState = atom({
  key: 'selectedTabState',
  default: 'NFTs',
})

export const subgraphDataArrayState = atom({
  key: 'subgraphData',
  default: [{}],
})

// Chart.tsx
export const paginatedPageNumState = atom({
  key: 'paginationPageNumState',
  default: 0,
})

export const subgraphQueryState = atom({
  key: 'subgraphqueryState',
  default: 'NFTs',
})

