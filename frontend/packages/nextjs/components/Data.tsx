import Chart from "./Chart";
import { useSubgraph } from "~~/hooks/scaffold-eth/useSubgraph";

const Data = () => {
  // what are the dimensions of the chart?
  // const data = useSubgraph({
  //   rows: 5,
  //   orderDirection: "asc",
  // });

  const sampleData = {
    addresses: [
      { id: "0x0ae2eA778dcA4489eFBF7ca8ecE6B9F876Be2B1c", numTxs: 4, isContract: false },
      { id: "0x0ae2eA778dcA4489eFBF7ca8ecE6B9F876Be2B1c", numTxs: 4, isContract: false },
      { id: "0x0ae2eA778dcA4489eFBF7ca8ecE6B9F876Be2B1c", numTxs: 4, isContract: false },
      { id: "0x0ae2eA778dcA4489eFBF7ca8ecE6B9F876Be2B1c", numTxs: 4, isContract: false },
      { id: "0x0ae2eA778dcA4489eFBF7ca8ecE6B9F876Be2B1c", numTxs: 4, isContract: false },
      { id: "0x0ae2eA778dcA4489eFBF7ca8ecE6B9F876Be2B1c", numTxs: 4, isContract: false },
      { id: "0x0ae2eA778dcA4489eFBF7ca8ecE6B9F876Be2B1c", numTxs: 4, isContract: false },
      { id: "0x0ae2eA778dcA4489eFBF7ca8ecE6B9F876Be2B1c", numTxs: 4, isContract: false },
      { id: "0x0ae2eA778dcA4489eFBF7ca8ecE6B9F876Be2B1c", numTxs: 4, isContract: false },
      { id: "0x0ae2eA778dcA4489eFBF7ca8ecE6B9F876Be2B1c", numTxs: 4, isContract: false },
    ],
  };

  return (
    <>
      <Chart sampleData={sampleData} />
    </>
  );
};

export default Data;
