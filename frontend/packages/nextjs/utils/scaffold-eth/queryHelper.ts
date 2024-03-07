import { queries, variables } from "./subgraphQueries";

export const queryHelper = (selectedTab: string, tabFilter: string = "default") => {
  switch (selectedTab) {
    case "Tokens":
      return { subgraphQuery: queries.defaultTokenQuery, variables: variables.default };
      break;
    case "NFTs":
      return { subgraphQuery: queries.defaultNFTQuery, variables: variables.default };
      break;
    default:
      return { subgraphQuery: queries.defaultTokenQuery, variables: variables.default };
      break;
  }
};
