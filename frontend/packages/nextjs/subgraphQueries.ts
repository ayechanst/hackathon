export const queries = {
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
        }
      }
    `,
  tokensByTimeQuery: `
      query tokenTimes($rows: Int, $by: String, $direction: String) {
        tokens(first: $rows, where: {name_not: "", symbol_not: ""}, orderBy: $by, orderDirection: $direction) {
            id
            name
            symbol
            decimals
            totalSupply
            volume
            count
        }
      }
    `,

  tokensByDay: `
      query tokenTimes($rows: Int) {
        tokens(first: $rows, where: {name_not: "", dayCount_not: null}, orderBy: dayCount, orderDirection: desc) {
            id
            name
            symbol
            decimals
            dayVolume
            dayCount
            volume
            count
        }
      }
    `,
  tokensByWeek: `
      query tokenTimes($rows: Int) {
        tokens(first: $rows, where: {name_not: "", weekCount_not: null}, orderBy: weekCount, orderDirection: desc) {
            id
            name
            symbol
            decimals
            weekVolume
            weekCount
            volume
            count
        }
      }
    `,
  tokensByMonth: `
      query tokenTimes($rows: Int) {
        tokens(first: $rows, where: {name_not: "", monthCount_not: null}, orderBy: monthCount, orderDirection: desc) {
            id
            name
            symbol
            decimals
            monthVolume
            monthCount
            volume
            count
        }
      }
    `,
  defaultNFTQuery: `
      query NFTs($rows: Int) {
          nfts(first: $rows, where: {name_not: null, symbol_not: ""}, orderBy: volume, orderDirection: desc){
            id
            name
            symbol
            volume
            dayVolume
            weekVolume
            monthVolume
        }
      }
    `,

  NFTsByTimeQuery: `
      query NFTTimes($rows: Int, $by: String, $direction: String) {
        nfts(first: $rows, where: {name_not: "", symbol_not: ""}, orderBy: $by, orderDirection: $direction) {
          id
          name
          symbol
          volume
        }
      }
    `,
  nftByDay: `
      query NFTTimes($rows: Int) {
        nfts(first: $rows, where: {name_not: "", dayVolume_not: null}, orderBy: dayVolume, orderDirection: desc) {
          id
          name
          volume
          dayVolume
      }
      }`,
  nftByWeek: `
      query NFTTimes($rows: Int) {
        nfts(first: $rows, where: {name_not: "", weekVolume_not: null}, orderBy: weekVolume, orderDirection: desc) {
          id
          name
          volume
          weekVolume
      }
      }`,
  nftByMonth: `
      query NFTTimes($rows: Int) {
        nfts(first: $rows, where: {name_not: "", monthVolume_not: null}, orderBy: monthVolume, orderDirection: desc) {
          id
          name
          volume
          monthVolume
      }
      }`,

  tokenSearchQuery: `
      query tokenSearch($address: String) {
        tokens(first: 1, where: {id: $address}, orderBy: count, orderDirection: desc){
            id
            name
            symbol
            totalSupply
            decimals
            volume
            count
            monthVolume
            monthCount
        }
      }
    `,

  nftSearchQuery: `
      query nftSearch($address: String) {
          nftTokens(first: 10, where: {address: $address}, orderBy: volume, orderDirection: desc){
            tokenId
            volume
          }
        }
    `,
};

export const variables = {
  default: {
    rows: 10,
  },
  timestampDescending: {
    rows: 10,
    by: "deploymentTimestamp",
    direction: "desc",
  },
  timestampAscending: {
    rows: 10,
    by: "deploymentTimestamp",
    direction: "asc",
  },
  tokensDay: {
    rows: 10,
    by: "dayCount",
    direction: "desc",
  },
};
