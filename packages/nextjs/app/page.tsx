"use client";

import Link from "next/link";
import type { NextPage } from "next";
import { BugAntIcon, MagnifyingGlassIcon } from "@heroicons/react/24/outline";
import Data from "~~/components/Data";

const Home: NextPage = () => {
  return (
    <>
      <div className="flex justify-center">
        <Data />
      </div>
    </>
  );
};

export default Home;
