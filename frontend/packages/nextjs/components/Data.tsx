import Menu from "./Menu";
import Tabs from "./Tabs";

export type Query = {
  subgraphQuery: string;
  variables: {
    rows: number;
    contract?: string;
    blockNumber?: number;
    timestamp?: number;
    timestamp_gte?: number;
    timestamp_lt?: number;
    orderBy?: string;
    orderDirection?: string;
  };
};

const Data = () => {
  return (
    <>
      <div className="grid grid-cols-10">
        <div className="mt-2 mx-5 col-span-2">
          <Menu />
        </div>
        <div className="mr-5 mt-2 col-span-8">
          <Tabs />
        </div>
        {/* <div className="mr-5 mt-2 col-span-1"></div> */}
      </div>
    </>
  );
};

export default Data;
