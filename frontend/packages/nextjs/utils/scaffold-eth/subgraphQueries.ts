export const queries = {
  defaultTokenQuery: `
            query tokenDeployment($rows: Int) {
                tokenDeployments(first: $rows, orderBy: blocknumber, orderDirection: desc) {
                    id
                    name
                    symbol
                    blocknumber
                }
                }
            `,
  defaultNFTQuery: `
            query NFTs($rows: Int) {
                nftDeployments(first: $rows, orderBy: blocknumber, orderDirection: desc) {
                    id
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
    orderBy: "timestamp",
    orderDirection: "desc",
  },
  timestampAscending: {
    rows: 80,
    orderBy: "timestamp",
    orderDirection: "asc",
  },
};
