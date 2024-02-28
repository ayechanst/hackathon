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
      <div className="bg-primary h-screen">
        <Nav />
        <div className="flex h-screen justify-between">
          <div className="mt-2 ml-5">
            <Menu />
          </div>
          <div className="justify-center mr-5">
            <Data />
          </div>
        </div>
      </div>
    </>
  );
};

export default Home;
