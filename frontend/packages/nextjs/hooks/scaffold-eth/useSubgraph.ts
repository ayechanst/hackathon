
import { useEffect, useState } from "react";

// jj code
// type UseSubgraphProps = {
  //   rows: number;
  //   orderDirection: "asc" | "desc";
  // };
  
  type UseSubgraphProps = {
    subgraphQuery: string;
  };
  
  // export function useSubgraph({ rows, orderDirection }: UseSubgraphProps) {
    export function useSubgraph({ subgraphQuery }: UseSubgraphProps) {
      // change the endpoint if accessing a different subgraph
      const endpoint = "https://api.studio.thegraph.com/query/64372/debbie-hack/0.0.2";
      const [data, setData] = useState<any>(null);
      
      const erc20Transfers = `erc20Transfers(first: 5) {
            id
            from
            to
            amount
          }`;
  
  const erc721Transfers = `erc721Transfers(first: \${{rows}} ) {
    id
    from
    to
    tokenId
    blocknumber
  }`;
  
  const erc20Deployments = `erc20Deployments(first: 5) {
          id
          name
          symbol
          totalSupply
        }`;
      
  const erc721Deployments = `erc721Deployments(first: 5) {
      id
      name
      symbol
      blocknumber
    }`;

      let query;
      // change the line below to something changable
      // let fetchSubgraphSetData = "erc20Transfers";
      let fetchSubgraphSetData;
    
      if (subgraphQuery == "erc20Transfers") {
        query = erc20Transfers;
        fetchSubgraphSetData = erc20Transfers;
      } else if (subgraphQuery == "erc721Transfers") {
        query = erc721Transfers;
        fetchSubgraphSetData = erc721Transfers;
      } else if (subgraphQuery == "erc20Deployments") {
        query = erc20Deployments;
        fetchSubgraphSetData = erc20Deployments;
      } else {
        query = erc721Deployments;
        fetchSubgraphSetData = erc721Deployments;
      };

      const queryAddys = query;
      
      const fetchSubgraph = async (query: String) => {
        const response = await fetch(endpoint, {
          method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ query }),
    });

    const { data } = await response.json();
    console.log(data);
    // change this each time
    setData(data.fetchSubgraphSetData);
  };
  
  useEffect(() => {
    fetchSubgraph(queryAddys).then(data => console.log(data));
  }, []);

  return { data };
}

// depositors(first: ${rows}, orderBy: rsEthMinted, orderDirection: ${orderDirection}) {
//     id
//     rsEthMintedReadable
//     stETHReadable
//     sfrxETHReadable
//     ETHxReadable
//   }