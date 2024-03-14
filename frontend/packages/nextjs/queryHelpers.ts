import { variables, queries } from "./subgraphQueries";

export const queryHelper = (selectedTab: string, filter: string = "default", timeFilter: string = "default") => {
  if (selectedTab === "Tokens") {
    if (filter === "oldest") {
      return { subgraphQuery: queries.tokensByTimeQuery, variables: variables.timestampAscending };
    } else if (filter === "newest") {
      return { subgraphQuery: queries.tokensByTimeQuery, variables: variables.timestampDescending };
    } else {
      return { subgraphQuery: queries.defaultTokenQuery, variables: variables.default };
    }
  } else if (selectedTab === "NFTs") {
    if (filter === "oldest") {
      return { subgraphQuery: queries.NFTsByTimeQuery, variables: variables.timestampAscending };
    } else if (filter === "newest") {
      return { subgraphQuery: queries.NFTsByTimeQuery, variables: variables.timestampDescending };
    } else {
      return { subgraphQuery: queries.defaultNFTQuery, variables: variables.default };
    }
  } else {
    return { subgraphQuery: queries.defaultTokenQuery, variables: variables.default };
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
  