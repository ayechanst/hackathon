import { useEffect, useState } from "react";

type UseSubgraphProps = {
  subgraphQuery: string;
};

const erc20Transfers = `
  query {
    erc20Transfers(first: 5) {
    id
    from
    to
    amount
  }
}`;

const erc721Transfers = `
  query {
    erc721Transfers(first: 5) {
    id
    from
    to
    tokenId
    blocknumber
  }
}`;

const erc20Deployments = `
  query {
    erc20Deployments(first: 5) {
    id
    name
    symbol
    totalSupply
  }
}`;

const erc721Deployments = `
query {
  erc721Deployments(first: 5) {
    id
    name
    symbol
    blocknumber
  }
}
`;

export function useSubgraph({ subgraphQuery }: UseSubgraphProps) {
  const [data, setData] = useState<any>(null);

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

    fetchSubgraph(currentQuery).then(data => console.log(data));
  }, [subgraphQuery]);

  const fetchSubgraph = async (query: string) => {
    const response = await fetch("https://api.studio.thegraph.com/query/64372/debbie-hack/0.0.2", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ query }),
    });

    const { data } = await response.json();
    const keys = Object.keys(data);
    setData(data[keys[0]]);
  };

  return { data };
}
