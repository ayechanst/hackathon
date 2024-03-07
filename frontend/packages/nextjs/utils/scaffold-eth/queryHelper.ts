import { queries, variables } from "./subgraphQueries";

export const queryHelper = (selectedTab: string, tabFilter: string = "default") => {
  switch (selectedTab) {
    case "Tokens":
      switch (tabFilter) {
        case "oldest":
          return { subgraphQuery: queries.tokensByTimeQuery, variables: variables.timestampAscending };
          break;
        case "newest":
          return { subgraphQuery: queries.tokensByTimeQuery, variables: variables.timestampDescending };
          break;
        default:
          return { subgraphQuery: queries.defaultTokenQuery, variables: variables.default };
          break;
      }
    case "NFTs":
      switch (tabFilter) {
        case "oldest":
          return { subgraphQuery: queries.NFTsByTimeQuery, variables: variables.timestampAscending };
          break;
        case "newest":
          return { subgraphQuery: queries.NFTsByTimeQuery, variables: variables.timestampDescending };
          break;
        default:
          return { subgraphQuery: queries.defaultNFTQuery, variables: variables.default };
          break;
      }
    default:
      return { subgraphQuery: queries.defaultTokenQuery, variables: variables.default };
      break;
  }
};
