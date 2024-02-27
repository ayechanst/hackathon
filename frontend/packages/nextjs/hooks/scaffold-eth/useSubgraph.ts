import { useEffect, useState } from "react";

type UseSubgraphProps = {
  rows: number;
  orderDirection: "asc" | "desc";
};

export function useSubgraph({ rows, orderDirection }: UseSubgraphProps) {
  const endpoint = "https://api.studio.thegraph.com/query/64372/kelpdao-sps/0.2.3";
  const [data, setData] = useState<any>(null);

  const queryAddys = `
  query {
  depositors(first: ${rows}, orderBy: rsEthMinted, orderDirection: ${orderDirection}) {
    id
    rsEthMintedReadable
    stETHReadable
    sfrxETHReadable
    ETHxReadable
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
    setData(data.depositors);
  };

  useEffect(() => {
    fetchSubgraph(queryAddys).then(data => console.log(data));
  }, []);

  return { data };
}
