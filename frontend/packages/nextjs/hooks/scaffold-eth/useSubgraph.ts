import { useEffect, useState } from "react";

// jj code
// type UseSubgraphProps = {
  //   rows: number;
  //   orderDirection: "asc" | "desc";
  // };
  
type UseSubgraphProps = {
  subgraphQuery: string;
};
  
export function useSubgraph({ subgraphQuery }: UseSubgraphProps) {
  const endpoint =
  "https://api.studio.thegraph.com/query/64372/debbie-hack/0.0.2";
  const [data, setData] = useState<any>(null);
  const [query, setQuery] = useState<string>("");

  const erc20Transfers = `erc20Transfers(first: 5) {
    id
    from
    to
    amount
  }`;

  const erc721Transfers = `erc721Transfers(first: 5) {
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

  useEffect(() => {
    let currentQuery = "";
    if (subgraphQuery == "erc20Transfers") {
      currentQuery = erc20Transfers;
    } else if (subgraphQuery == "erc721Transfers") {
      currentQuery = erc721Transfers;
    } else if (subgraphQuery == "erc20Deployments") {
      currentQuery = erc20Deployments;
    } else {
      currentQuery = erc721Deployments;
    }
    setQuery(currentQuery);
    console.log("currentQuerry from hook", currentQuery);

    const fetchSubgraph = async (query: string) => {
      const response = await fetch(endpoint, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ query }),
      });

      const { data } = await response.json();
      console.log("data from hook", data);
      console.log("query from hook", query)

      setData(data);
    };

    // before
    //   fetchSubgraph(currentQuery).then((data) => console.log(data));
    // }, [subgraphQuery]);
    // after
    fetchSubgraph(query).then((data) => console.log(data));
  }, [subgraphQuery]);
  
  return { data };
}

// export function useSubgraph({ rows, orderDirection }: UseSubgraphProps) {
// export function useSubgraph({ subgraphQuery }: UseSubgraphProps) {
//   // change the endpoint if accessing a different subgraph
//   const endpoint = "https://api.studio.thegraph.com/query/64372/debbie-hack/0.0.2";
//   const [data, setData] = useState<any>(null);
//   const [query, setQuery] = useState<string>("");
//   const [fetchSubgraphData, setFetchSubgraphData] = useState<string>("");
    
//     // change the line below to something changable
//     // let fetchSubgraphSetData = "erc20Transfers";
//   let fetchSubgraphSetData;

//   const erc20Transfers = `erc20Transfers(first: 5) {
//     id
//     from
//     to
//     amount
//   }`;

//   const erc721Transfers = `erc721Transfers(first: \${{rows}} ) {
//     id
//     from
//     to
//     tokenId
//     blocknumber
//   }`;

//   const erc20Deployments = `erc20Deployments(first: 5) {
//     id
//     name
//     symbol
//     totalSupply
//   }`;

//   const erc721Deployments = `erc721Deployments(first: 5) {
//     id
//     name
//     symbol
//     blocknumber
//   }`;
  
//   if (subgraphQuery == "erc20Transfers") {
//     setQuery(erc20Transfers);
//     setFetchSubgraphData("erc20Transfers");
//   } else if (subgraphQuery == "erc721Transfers") {
//     setQuery(erc721Transfers);
//     setFetchSubgraphData("erc721Transfers");
//   } else if (subgraphQuery == "erc20Deployments") {
//     setQuery(erc20Deployments);
//     setFetchSubgraphData("erc20Deployments");
//   } else {
//     setQuery(erc721Deployments);
//     setFetchSubgraphData("erc721Deployments");
//   };
  
//   const fetchSubgraph = async (query: String) => {
//     const response = await fetch(endpoint, {
//       method: "POST",
//       headers: {
//         "Content-Type": "application/json",
//       },
//     body: JSON.stringify({ query }),
//     });

//     const { data } = await response.json();
//     console.log(data);
//   // change this each time
//     setData(data[fetchSubgraphData]);
//   };
  
//   useEffect(() => {
//     fetchSubgraph(query).then(data => console.log(data));
//   }, []);

//   return { data };
// }

// depositors(first: ${rows}, orderBy: rsEthMinted, orderDirection: ${orderDirection}) {
  //     id
  //     rsEthMintedReadable
//     stETHReadable
//     sfrxETHReadable
//     ETHxReadable
//   }