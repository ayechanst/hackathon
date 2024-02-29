const SearchInput = () => {
  return (
    <>
      <div className="rounded-lg bg-accent text-primary w-full m-1">
        {/* removed "input" from label's class name */}
        <label className="flex items-center text-primary m-1">
          <input type="text" className="bg-accent text-primary mx-1 border-none" placeholder="Search" />
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 16 16"
            fill="currentColor"
            className="w-4 h-4 opacity-100"
          >
            <path
              fillRule="evenodd"
              d="M9.965 11.026a5 5 0 1 1 1.06-1.06l2.755 2.754a.75.75 0 1 1-1.06 1.06l-2.755-2.754ZM10.5 7a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0Z"
              clipRule="evenodd"
            />
          </svg>
        </label>
      </div>
    </>
  );
};
export default SearchInput;
