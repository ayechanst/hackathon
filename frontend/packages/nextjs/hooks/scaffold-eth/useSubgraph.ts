import { useEffect, useState } from "react";

type UseSubgraphProps = {
  rows: number;
  orderDirection: "asc" | "desc";
};

export function useSubgraph({ rows, orderDirection }: UseSubgraphProps) {
  // change the endpoint if accessing a different subgraph
  const endpoint = "https://api.studio.thegraph.com/query/64372/debbie-hack/0.0.2";
  const [data, setData] = useState<any>(null);

  // erc721Transfers(first: ${rows} ){
  //   id
  //   from
  //   to
  //   tokenId
  //   blocknumber
  
  // depositors(first: ${rows}, orderBy: rsEthMinted, orderDirection: ${orderDirection}) {
    //   id
    //   rsEthMintedReadable
    //   stETHReadable
    //   sfrxETHReadable
    //   ETHxReadable
    // }
    
    //   erc20Deployments(first: 5) {
      //     id
      //     name
      //     symbol
      //     totalSupply
      //   }

  //   erc721Deployments(first: 5) {
  //     id
  //     name
  //     symbol
  //     blocknumber
  //   }
  
  //   erc20Transfers(first: 5) {
    //     id
    //     from
    //     to
    //     amount
    //   }
    const queryAddys = `
    query {
      erc20Transfers(first: 5, where: { id: "436bfe4380f2ce5535345e430d1d24849ac0d069" }, orderBy: amount, orderDirection: desc) {
        id
        from
        to
        amount
      }
    }`;
    
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
    setData(data.erc20Transfers);
  };

  useEffect(() => {
    fetchSubgraph(queryAddys).then(data => console.log(data));
  }, []);

  return { data };
}
