"use client";

import React from "react";
import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client";
import type { NextPage } from "next";
import { RecoilRoot } from "recoil";
// import { BugAntIcon, MagnifyingGlassIcon } from "@heroicons/react/24/outline";
import Data from "~~/components/Data";
// import Menu from "~~/components/Menu";
import Nav from "~~/components/Nav";

const Home: NextPage = () => {
  const client = new ApolloClient({
    uri: "https://api.studio.thegraph.com/query/64372/debbie-hack/0.0.2",
    cache: new InMemoryCache(),
  });

  return (
    <RecoilRoot>
      <ApolloProvider client={client}>
        <div className="bg-primary">
          <Nav />
          <div className="w-screen">
            <Data />
          </div>
        </div>
      </ApolloProvider>
    </RecoilRoot>
  );
};

export default Home;
