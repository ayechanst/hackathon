import React, { useState } from "react";
import { motion } from "framer-motion";

interface SearchInputProps {
  onFormSubmit: (value: string) => void;
}

const SearchInput: React.FC<SearchInputProps> = ({ onFormSubmit }) => {
  const [inputValue, setInputValue] = useState("");
  const [isFocused, setIsFocused] = useState(false);
  const [isActive, setIsActive] = useState(false);

  function handleInputChange(event: React.ChangeEvent<HTMLInputElement>) {
    setInputValue(event.target.value);
  }

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (onFormSubmit) {
      onFormSubmit(inputValue);
    }
  }

  return (
    <motion.form
      className={`rounded-lg w-full m-2`}
      onSubmit={handleSubmit}
      animate={{
        backgroundColor: isActive ? "#27272a" : "#eab308",
        color: isActive ? "#eab308" : "#27272a",
        boxShadow: isActive ? "0 0 10px 3px rgba(66, 153, 225, 0.5)" : "none",
        scale: isActive ? 1.025 : 1,
      }}
      whileHover={{ scale: 1.025 }}
      whileFocus={{ scale: 1.1 }}
      transition={{ duration: 0.3 }}
      onBlur={() => setIsActive(false)}
    >
      <label className="flex items-center text-primary m-1">
        <input
          type="text"
          className={`bg-transparent text-primary m-1 placeholder-primary w-full outline-none ${
            isFocused ? "text-yellow-500" : ""
          }`}
          placeholder="Search"
          value={inputValue}
          onChange={handleInputChange}
          onFocus={() => {
            setIsFocused(true);
            setIsActive(true);
          }}
          onBlur={() => {
            setIsFocused(false);
            setIsActive(false);
          }}
        />
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" className="w-4 h-4 opacity-100">
          <path
            fillRule="evenodd"
            d="M9.965 11.026a5 5 0 1 1 1.06-1.06l2.755 2.754a.75.75 0 1 1-1.06 1.06l-2.755-2.754ZM10.5 7a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0Z"
            clipRule="evenodd"
          />
        </svg>
      </label>
    </motion.form>
  );
};

export default SearchInput;
