import { queries, variables } from "./subgraphQueries";

export const queryHelper = (selectedTab: string, filter: string, timeFilter: string, searchInput: string | void) => {
  // console.log("from helper");
  // console.log("selectedTab", selectedTab);
  // console.log("filter", filter);
  // console.log("timeFilter", timeFilter);

  if (selectedTab === "Tokens") {
    if (filter === "volume") {
      if (timeFilter === "day") {
        return {
          subgraphQuery: queries.tokensByDay,
          variables: variables.default,
        };
      } else if (timeFilter === "week") {
        return {
          subgraphQuery: queries.tokensByWeek,
          variables: variables.tokensDay,
        };
      } else if (timeFilter === "month") {
        return {
          subgraphQuery: queries.tokensByMonth,
          variables: variables.tokensDay,
        };
      } else if (timeFilter === "all") {
        return {
          subgraphQuery: queries.defaultTokenQuery,
          variables: variables.default,
        };
      } else {
        return {
          subgraphQuery: queries.defaultTokenQuery,
          variables: variables.default,
        };
      }
    } else if (filter === "newest") {
      return {
        subgraphQuery: queries.defaultTokenQuery,
        variables: variables.timestampDescending,
      };
    } else if (filter == "oldest") {
      return {
        subgraphQuery: queries.tokensByTimeQuery,
        variables: variables.timestampAscending,
      };
    } else if (searchInput != "" && searchInput != undefined) {
      return {
        subgraphQuery: queries.tokenSearchQuery,
        variables: { address: searchInput },
      };
    } else {
      return {
        subgraphQuery: queries.defaultTokenQuery,
        variables: variables.default,
      };
    }
  } else if (selectedTab === "NFTs") {
    if (filter === "oldest") {
      return {
        subgraphQuery: queries.NFTsByTimeQuery,
        variables: variables.timestampAscending,
      };
    } else if (filter === "newest") {
      return {
        subgraphQuery: queries.NFTsByTimeQuery,
        variables: variables.timestampDescending,
      };
    } else if (filter === "volume") {
      if (timeFilter === "day") {
        return {
          subgraphQuery: queries.nftByDay,
          variables: variables.default,
        };
      } else if (timeFilter === "week") {
        return {
          subgraphQuery: queries.nftByWeek,
          variables: variables.default,
        };
      } else if (timeFilter === "month") {
        return {
          subgraphQuery: queries.nftByMonth,
          variables: variables.default,
        };
      } else {
        return {
          subgraphQuery: queries.defaultNFTQuery,
          variables: variables.default,
        };
      }
    } else if (searchInput != "" && searchInput != undefined) {
      return {
        subgraphQuery: queries.nftSearchQuery,
        variables: { address: searchInput },
      };
    } else {
      return {
        subgraphQuery: queries.defaultNFTQuery,
        variables: variables.default,
      };
    }
  } else {
    return {
      subgraphQuery: queries.defaultTokenQuery,
      variables: variables.default,
    };
  }
};

export type Query = {
  subgraphQuery: string;
  variables: any;
  // variables: {
  //   rows: number;
  //   contract?: string;
  //   blockNumber?: number;
  //   timestamp?: number;
  //   timestamp_gte?: number;
  //   timestamp_lt?: number;
  //   orderBy?: string;
  //   orderDirection?: string;
  // };
};
