"use client";

import React from "react";
import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client";
import type { NextPage } from "next";
import Content from "~~/components/Content";
import Nav from "~~/components/Nav";

const Home: NextPage = () => {
  const client = new ApolloClient({
    // uri: "https://api.studio.thegraph.com/query/55738/hackathontest/0.0.1",
    uri: "https://api.studio.thegraph.com/query/64372/data-detective/0.1.1",
    headers: {},
    cache: new InMemoryCache(),
  });

  return (
    <ApolloProvider client={client}>
      <div className="bg-primary">
        <Nav />
        <div className="w-screen">
          <Content />
        </div>
      </div>
    </ApolloProvider>
  );
};

export default Home;
