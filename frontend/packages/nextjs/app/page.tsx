"use client";

import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client";
import type { NextPage } from "next";
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
    <ApolloProvider client={client}>
      <div className="bg-primary">
        <Nav />
        <div className="flex">
          <div className="mr-5 w-4/5">
            <Data />
          </div>
        </div>
      </div>
    </ApolloProvider>
  );
};

export default Home;
