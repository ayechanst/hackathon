"use client";

import Link from "next/link";
import type { NextPage } from "next";
import { BugAntIcon, MagnifyingGlassIcon } from "@heroicons/react/24/outline";
import Data from "~~/components/Data";
import Menu from "~~/components/Menu";
import Nav from "~~/components/Nav";

const Home: NextPage = () => {
  return (
    <>
      <div className="bg-primary">
        <Nav />
        <div className="flex">
          <div className="mt-2 mx-5 w-1/5">
            <Menu />
          </div>
          <div className="mr-5 w-4/5">
            <Data />
          </div>
        </div>
      </div>
    </>
  );
};

export default Home;
