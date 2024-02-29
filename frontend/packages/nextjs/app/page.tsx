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
        <div className="w-screen">
          <Data />
        </div>
      </div>
    </>
  );
};

export default Home;
