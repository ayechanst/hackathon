"use client";

import Link from "next/link";
import type { NextPage } from "next";
import { BugAntIcon, MagnifyingGlassIcon } from "@heroicons/react/24/outline";
import Data from "~~/components/Data";
import Nav from "~~/components/Nav";

const Home: NextPage = () => {
  return (
    <>
      <Nav />
      <div className="flex justify-center bg-[url('../public/bg.svg')] h-screen bg-no-repeat bg-cover">
        <Data />
      </div>
    </>
  );
};

export default Home;
