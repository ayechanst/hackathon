import Dropdown from "./Dropdown";

const Nav = () => {
  const dropdownListItems = ["one", "two", "three"];
  return (
    <>
      <div className="navbar bg-[url('../public/bg.svg')] bg-no-repeat bg-cover text-neutral-content max-w-screen">
        <div className="flex justify-between items-center">
          <button className="btn btn-ghost text-white text-xl">WabbaTrack</button>
          <div className="flex">
            <button className="btn">Button</button>
            <button className="btn">Button</button>
            <button className="btn">Button</button>
            <Dropdown dropdownName={"drop down no cap"} dropdownListItems={dropdownListItems} />
          </div>
        </div>
      </div>
    </>
  );
};
export default Nav;
