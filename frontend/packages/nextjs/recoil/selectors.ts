import { selector } from "recoil";
import { paginatedPageNumState, selectedTabState, subgraphDataArrayState, subgraphFilterQueryState } from "./atoms";


export const filterButtonsState = selector({
  key: 'filterButtonState', // unique ID (with respect to other atoms/selectors)
  get: ({get}) => {
    const clickedTab = get(selectedTabState);
    if (clickedTab === "NFTs") {
      return ["transfer volume"] ;
    } else if (clickedTab === "NFT Collection") {
      return ["transfer volume"] ;
    } else if (clickedTab === "NFT Holders") {
      return ["input component"];
    } else if (clickedTab === "Tokens") {
      return ["transfer volume", "relative transfer volume"];
    } else if (clickedTab === "Token Holders") {
      return ["input component", "token balance", "transfer volume"];
    } else {
      return ["invalid selection"];
    }
  },
});

export const dividedSubgraphDataState = selector({
  key: 'dividedSubgraphDataState',
  get: ({get}) => {
    const subgraphDataArray = get(subgraphDataArrayState);
    const paginatedPageNum = get(paginatedPageNumState);
    let returnedValue = [];
    for (let i = 0; i < subgraphDataArray.length; i += 10) {
    returnedValue.push(subgraphDataArray.slice(i, i + 10));
    }
    return returnedValue[paginatedPageNum];
  }
})

// export const masterSubgraphQuery = selector({
//   key: 'masterSubgraphQuery',
//   get: ({get}) => {
//     const selectedTab = get(selectedTabState);
//     const selectedTime = get(subgraphFilterQueryState);
//     const selectedfilter = get(subgraphFilterQueryState);
//   }
// })
