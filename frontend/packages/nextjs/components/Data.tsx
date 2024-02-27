import Chart from "./Chart";
import { useSubgraph } from "~~/hooks/scaffold-eth/useSubgraph";

const Data = () => {
  const { data } = useSubgraph({
    rows: 20,
    orderDirection: "desc",
  });

  return (
    <>
      <Chart data={data} />
    </>
  );
};

export default Data;
