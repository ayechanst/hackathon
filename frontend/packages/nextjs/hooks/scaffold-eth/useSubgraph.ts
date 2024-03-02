import { useEffect, useState } from "react";
import { gql, useQuery } from "@apollo/client";
import { useRecoilValue } from "recoil";
import {
  selectedTabState,
  subgraphFilterQueryState,
  subgraphQueryState,
  subgraphTimeQueryState,
} from "~~/recoil/atoms";

// type UseSubgraphProps = {
//   subgraphQuery?: string;
//   queryProps?: any;
// };
type UseSubgraphProps = {
  subgraphQuery?: string;
  queryProps?: any;
};

export function useSubgraph({ subgraphQuery, queryProps }: UseSubgraphProps) {
  const queryPropsRecoil = useRecoilValue(subgraphFilterQueryState);
  useEffect(() => {
    console.log(queryPropsRecoil);
  }, [queryPropsRecoil]);
  // const subgraphTimeQuery = useRecoilValue(subgraphTimeQueryState);
  // const subgraphTabQuery = useRecoilValue(selectedTabState);
  // const subgraphQuery = useRecoilValue(subgraphFilterQueryState);

  const {
    loading,
    error,
    data: responseData,
  } = useQuery(getQuery(subgraphQuery!, queryPropsRecoil), {
    // variables take in the filters
    variables: { rows: 80 },
    // pollInterval: 10000,
  });

  const [data, setData] = useState<any>(null);

  useEffect(() => {
    if (!responseData) return;

    const keys = Object.keys(responseData);
    const getRidOfTypeName = responseData[keys[0]];

    const returnData = getRidOfTypeName.map((item: any) => {
      const { __typename, ...rest } = item;
      return rest;
    });

    console.log("returnData", returnData);

    setData(returnData);
  }, [responseData]);

  return { data, loading, error };
}

const getQuery = (subgraphQuery: string, queryProps: string) => {
  if (subgraphQuery === "NFTs") {
    if (queryProps === "Transfers") {
      return gql`
        query nftsTransfers($rows: Int) {
          nfts(first: $rows, orderBy: volume, orderDirection: desc) {
            id
            from
            to
            tokenId
            blocknumber
          }
        }
      `;
    }
    return gql`
      query nfts($rows: Int) {
        nfts(first: $rows, orderBy: volume, orderDirection: desc) {
          id
          name
          symbol
          volume
        }
      }
    `;
  } else if (subgraphQuery === "Tokens") {
    if (queryProps == "Transfer volume") {
      return gql`
        query tokens($rows: Int) {
          tokens(first: $rows, orderBy: volume, orderDirection: desc) {
            id
            name
            symbol
            volume
          }
        }
      `;
    } else if (queryProps == "Tx volume") {
      return gql`
        query tokens($rows: Int) {
          tokens(first: $rows, orderBy: count, orderDirection: desc) {
            id
            name
            symbol
            count
            volume
            totalSupply
            decimals
          }
        }
      `;
    } else {
      return gql`
        query tokens($rows: Int) {
          tokens(first: $rows, where: { name_not: null }, orderBy: count, orderDirection: desc) {
            id
            name
            symbol
            totalSupply
            decimals
          }
        }
      `;
    }
  } else if (subgraphQuery === "erc20Transfers") {
    return gql`
      query erc20Transfers($rows: Int) {
        erc20Transfers(first: $rows) {
          id
          from
          to
          amount
        }
      }
    `;
  } else if (subgraphQuery == "erc721Transfers") {
    return gql`
      query erc721Transfers($rows: Int) {
        erc721Transfers(first: $rows, orderBy: blocknumber, orderDirection: desc) {
          id
          from
          to
          tokenId
          blocknumber
        }
      }
    `;
  } else if (subgraphQuery == "erc20Deployments") {
    return gql`
      query erc20Deployments($rows: Int) {
        erc20Deployments(first: $rows) {
          id
          name
          symbol
          totalSupply
        }
      }
    `;
  } else {
    return gql`
      query erc721Deployments($rows: Int) {
        erc721Deployments(first: $rows) {
          id
          name
          symbol
          blocknumber
        }
      }
    `;
  }
};
