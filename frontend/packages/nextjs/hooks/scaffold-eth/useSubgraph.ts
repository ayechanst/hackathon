import { useEffect, useState } from "react";
import { gql, useQuery } from "@apollo/client";
import { useRecoilValue } from "recoil";
import {
  subgraphFilterQueryState,
  subgraphQueryState,
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

    const returnDataKeys = Object.keys(returnData[0]);
    if (returnDataKeys.includes("decimals") && returnDataKeys.includes("volume")) {
      const mapTokenVol = returnData.map((item: any) => {
        item.volume = item.volume / Math.pow(10, item.decimals);
        const { decimals, ...rest } = item;
        return rest;
      });
      if (returnDataKeys.includes("totalSupply")) {
        const mapTokenTotalSupply = mapTokenVol.map((item: any) => {
          if (!item.totalSupply) {
            item.totalSupply = "N/A";
            return item;
          }
        });
        setData(mapTokenTotalSupply);
      }
      setData(mapTokenVol);
      console.log("returnData", returnData);
    } else {
      setData(returnData);
    }
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
    } else if (queryProps === "New") {
      return gql`
        {
          nfts(first: 10, orderBy: nftDeployment__timestamp, orderDirection: desc) {
            id
            name
            symbol
            volume
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
    if (queryProps == "Tx volume") {
      return gql`
        query tokens($rows: Int) {
          tokens(first: $rows, where: { name_not: null, symbol_not: "" }, orderBy: count, orderDirection: desc) {
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
    } else if (queryProps == "New Tokens") {
      return gql`
        query tokens($rows: Int) {
          tokens(
            first: 10
            where: { name_not: null, symbol_not: "" }
            orderBy: tokenDeployment__timestamp
            orderDirection: desc
          ) {
            name
            symbol
            totalSupply
            count
            volume
            tokenDeployment {
              timestamp
            }
          }
        }
      `;
    } else {
      return gql`
        query tokens($rows: Int) {
          tokens(first: $rows, where: { name_not: null, symbol_not: "" }, orderBy: count, orderDirection: desc) {
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
