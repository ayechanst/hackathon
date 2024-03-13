export const queries = {
    // defaultTokenQuery: `
    //   query tokenDeployment($rows: Int) {
    //     tokenDeployments(first: $rows, where: {name_not: ""}, orderBy: blocknumber, orderDirection: desc) {
    //       id
    //       name
    //       symbol
    //       blocknumber
    //     }
    //   }
    // `,
    defaultTokenQuery: `
      query tokenDeployment($rows: Int) {
        tokens(first: $rows, where: {name_not: null, symbol_not: ""}, orderBy: count, orderDirection: desc){
            id
            name
            symbol
            totalSupply
            decimals
            volume
            count
            dayCount
            dayVolume
            weekCount
            weekVolume
            monthCount
            monthVolume
        }
      }
    `,
    tokensByTimeQuery: `
      query tokenTimes($rows: Int, $by: String, $direction: String) {
        tokenDeployments(first: $rows, where: {name_not: "", symbol_not: ""}, orderBy: $by, orderDirection: $direction) {
          id
          name
          symbol
          blocknumber
        }
      }
    `,
    defaultNFTQuery: `
      query NFTs($rows: Int) {
        nftDeployments(first: $rows, where: {name_not: "", symbol_not: "", tokenUri_not: ""}, orderBy: blocknumber, orderDirection: desc) {
          id
          name
          symbol
          blocknumber
        }
      }
    `,
  
    NFTsByTimeQuery: `
      query NFTTimes($rows: Int, $by: String, $direction: String) {
        nftDeployments(first: $rows, where: {name_not: "", symbol_not: ""}, orderBy: $by, orderDirection: $direction) {
          id
          name
          blocknumber
        }
      }
    `,
  };
  
  export const variables = {
    default: {
      rows: 80,
    },
    timestampDescending: {
      rows: 80,
      by: "timestamp",
      direction: "desc",
    },
    timestampAscending: {
      rows: 80,
      by: "timestamp",
      direction: "asc",
    },
  };