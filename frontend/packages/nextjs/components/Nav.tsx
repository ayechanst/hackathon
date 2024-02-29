import { useEffect, useState } from "react";
import { motion } from "framer-motion";

const Nav = () => {
  const [isPulsating, setIsPulsating] = useState(true);

  useEffect(() => {
    const interval = setInterval(() => {
      setIsPulsating(prevState => !prevState);
    }, 500); // 1000 milliseconds = 1 seconds
    return () => clearInterval(interval);
  }, []);

  return (
    <>
      <motion.div
        animate={{
          boxShadow: isPulsating ? "0 0 10px 3px rgba(66, 153, 225, 0.5)" : "0 0 0px 0px rgba(0, 0, 0, 0)", // Toggle between pulsating and no shadow
        }}
        transition={{ duration: 2, ease: "easeInOut" }}
        className="mx-5 mt-5 mb-2 bg-zinc-700 rounded-lg shadow-xl"
      >
        <div className="navbar text-neutral-content">
          <div className="flex justify-between items-center">
            <div>
              <img src="/fingerprint.svg" alt="Your SVG" className="h-11 w-11 text-yellow-100" />
            </div>
            <button className="btn btn-ghost text-yellow-100 text-3xl">Data Detective</button>
          </div>
          <div>
            First ever open source, 0 rpc call, pre-compiled, multi-architectural, blockchain, substream powered subraph
            querying, data indexer
          </div>
        </div>
      </motion.div>
    </>
  );
};
export default Nav;
