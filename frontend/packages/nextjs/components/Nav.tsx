import Breadcrumbs from "./Breadcrumbs";
import Dropdown from "./Dropdown";

const Nav = () => {
  return (
    <>
      <div className="m-5 bg-zinc-700 rounded-lg shadow-xl">
        <div className="navbar text-neutral-content">
          <div className="flex justify-between items-center">
            <button className="btn btn-ghost text-white text-xl">Dr. Data Doctor</button>
          </div>
          <Breadcrumbs breadcrumbList={["do", "re", "mi", "fa", "sol", "la", "ti"]} />
        </div>
      </div>
    </>
  );
};
export default Nav;
