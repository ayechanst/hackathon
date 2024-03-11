"use client";

import React from "react";
import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client";
import type { NextPage } from "next";
import { RecoilRoot } from "recoil";
import Data from "~~/components/Data";
import Nav from "~~/components/Nav";
import Tabs from "~~/components/Tabs";

const Home: NextPage = () => {
  const client = new ApolloClient({
    uri: "https://api.studio.thegraph.com/query/64372/data-detective/0.1.0",
    cache: new InMemoryCache(),
  });

  return (
    <RecoilRoot>
      <ApolloProvider client={client}>
        <div className="bg-primary">
          <Nav />
          <div className="w-screen">
            <Tabs />
          </div>
        </div>
      </ApolloProvider>
    </RecoilRoot>
  );
};

export default Home;
