// import { selector } from "recoil";
// import { paginatedPageNumState, selectedTabState, subgraphFilterQueryState } from "./atoms";


// export const dividedSubgraphDataState = selector({
//   key: 'dividedSubgraphDataState',
//   get: ({get}) => {
//     const subgraphDataArray = get(subgraphDataArrayState);
//     const paginatedPageNum = get(paginatedPageNumState);
//     let returnedValue = [];
//     for (let i = 0; i < subgraphDataArray.length; i += 10) {
//     returnedValue.push(subgraphDataArray.slice(i, i + 10));
//     }
//     return returnedValue[paginatedPageNum];
//   }
// })
