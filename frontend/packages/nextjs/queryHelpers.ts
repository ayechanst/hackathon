import { variables, queries } from "./subgraphQueries";

export const queryHelper = (selectedTab: string, tabFilter: string = "default", timeFilter: string = "default") => {
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

export type Query = {
    subgraphQuery: string;
    variables: {
      rows: number;
      contract?: string;
      blockNumber?: number;
      timestamp?: number;
      timestamp_gte?: number;
      timestamp_lt?: number;
      orderBy?: string;
      orderDirection?: string;
    };
  };
  