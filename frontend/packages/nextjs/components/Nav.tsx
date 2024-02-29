import Breadcrumbs from "./Breadcrumbs";

const Nav = () => {
  return (
    <>
      <div className="m-5 bg-zinc-700 rounded-lg shadow-xl">
        <div className="navbar text-neutral-content">
          <div className="flex justify-between items-center">
            <div>
              <img src="/fingerprint.svg" alt="Your SVG" className="h-11 w-11 text-yellow-100" />
            </div>
            <button className="btn btn-ghost text-yellow-100 text-3xl">Data Detective</button>
          </div>
          {/* <Breadcrumbs breadcrumbList={["do", "re", "mi", "fa", "sol", "la", "ti"]} /> */}
          <div>
            First ever open source, 0 rpc call, pre-compiled, multi-architectural, blockchain, substream powered subraph
            querying, data indexer
          </div>
        </div>
      </div>
    </>
  );
};
export default Nav;
