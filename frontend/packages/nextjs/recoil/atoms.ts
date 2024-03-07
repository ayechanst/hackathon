import { atom } from "recoil";
import { Query } from "~~/components/Data";

export const searchInputQueryState = atom({
  key: "searchInputQueryState", // unique ID (with respect to other atoms/selectors)
  default: "", // default value (aka initial value)
});

export const queryState = atom<Query>({
  key: "queryState",
  default: {
    subgraphQuery: `
    query tokenDeployment($rows: Int) {
        tokenDeployments(first: $rows, orderBy: blocknumber, orderDirection: desc) {
          id
          name
          symbol
          blocknumber
        }
      }
    `,
    variables: {
      rows: 80,
    },
  },
});

// this sets the props in the query hook
export const subgraphFilterQueryState = atom({
  key: "subgraphQueryState",
  default: "erc20Transfers",
});

export const filterButtonsArrayState = atom({
  key: "filterButtonsArrayState",
  default: ["transfer volume"],
});

export const subgraphTimeQueryState = atom({
  key: "subgraphTimeQueryState",
  default: "1 day",
});

// Tabs.tsx

export const selectedTabState = atom({
  key: "selectedTabState",
  default: "Tokens",
});

export const subgraphDataArrayState = atom({
  key: "subgraphData",
  default: [{}],
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
