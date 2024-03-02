import {
  paginatedPageNumState,
  searchInputQueryState,
  selectedTabState,
  subgraphDataArrayState,
  subgraphFilterQueryState,
} from "./atoms";
import { selector } from "recoil";

export const filterButtonsState = selector({
  key: "filterButtonState", // unique ID (with respect to other atoms/selectors)
  get: ({ get }) => {
    const clickedTab = get(selectedTabState);
    if (clickedTab === "NFTs") {
      return ["Transfers", "New"];
    } else if (clickedTab === "NFT Collection") {
      return ["Transfer volume"];
    } else if (clickedTab === "NFT Holders") {
      return ["input component"];
    } else if (clickedTab === "Tokens") {
      return ["Tx volume", "New Tokens"];
    } else if (clickedTab === "Token Holders") {
      return ["input component", "token balance", "transfer volume"];
    } else {
      return ["invalid selection"];
    }
  },
});

export const dividedSubgraphDataState = selector({
  key: "dividedSubgraphDataState",
  get: ({ get }) => {
    const subgraphDataArray = get(subgraphDataArrayState);
    const paginatedPageNum = get(paginatedPageNumState);
    let returnedValue = [];
    for (let i = 0; i < subgraphDataArray.length; i += 10) {
      returnedValue.push(subgraphDataArray.slice(i, i + 10));
    }
    return returnedValue[paginatedPageNum];
  },
});

// export const masterSubgraphQueryState = selector({
//   key: 'masterSubgraphQuery',
//   get: ({get}) => {
//     const selectedTab = get(selectedTabState);
//     const selectedTime = get(subgraphFilterQueryState);
//     const selectedFilter = get(subgraphFilterQueryState);
//     const inputAddressFilter = get(searchInputQueryState);
//     if (selectedTab === "NFTs") {
//       // some kind of query format
//       nfts(first: 10, orderBy: volume, orderDirection: desc) {
//     id
//     name
//     symbol
//     volume
//   }
//     } else if (selectedTab === "NFT Collection") {
// // code
//     } else if (selectedTab === "NFT Holders") {
// // code
//     } else if (selectedTab === "Tokens") {
// // code
//     } else if (selectedTab === "Token Holders") {
//      // code
//     } else {
//       // code
//     }
//   }
// })
