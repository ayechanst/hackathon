
import Menu from "./Menu";
import Tabs from "./Tabs";
import TimeMenu from "./TimeMenu";


// const Data = () => {
//   const [tabNames, setTabNames] = useState<string[]>([
//     "NFT",
//     "NFT Holders",
//     "NFT Collections",
//     "Tokens",
//     "Token Holders",
//   ]);

//   return (
//     <>
//       <Tabs tabName={tabNames} />
//     </>
//   );
// };

// export default Data;
const Data = () => {

  return (
    <>
      <div className="grid grid-cols-10">
        <div className="mt-2 mx-5 col-span-2">

          <Menu />
        </div>
        <div className="mr-5 mt-2 col-span-7">
          <Tabs />
        </div>
        <div className="mr-5 mt-2 col-span-1">
          {/* <TimeMenu /> */}

        </div>
      </div>
    </>
  );
};

export default Data;
