import { useEffect, useState } from "react";
import { gql, useQuery } from "@apollo/client";
import { subgraphQueryState, subgraphTimeQueryState } from "~~/recoil/atoms";
import { useRecoilValue } from "recoil";

type UseSubgraphProps = {
  subgraphQuery?: string;
  queryProps?: any;
};

export function useSubgraph({ subgraphQuery, queryProps }: UseSubgraphProps) {

  // use this in combination with subgraphQuery
  const subgraphTimeQuery = useRecoilValue(subgraphTimeQueryState);

  const {
    loading,
    error,
    data: responseData,
  } = useQuery(getQuery(subgraphQuery), {
    variables: queryProps,
    pollInterval: 10000,
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

    setData(returnData);
  }, [responseData]);

  return { data, loading, error };
}

const getQuery = (subgraphQuery: string) => {
  if (subgraphQuery === "erc20Transfers") {
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
  } else if (subgraphQuery == "erc721Deployments") {
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
