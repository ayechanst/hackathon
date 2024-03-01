import { selector } from "recoil";
import { selectedTabState } from "./atoms";



export const filterButtonsState = selector({
  key: 'charCountState', // unique ID (with respect to other atoms/selectors)
  get: ({get}) => {
    const clickedTab = get(selectedTabState);
    if (clickedTab === "NFTs") {
      return ["transfer volume"] ;
    } else if (clickedTab === "NFT Collection") {
      return ["transfer volume"] ;
    } else if (clickedTab === "NFT Holders") {
      return ["input component"];
    } else if (clickedTab === "Tokens") {
      return ["transfer volume", "relative transfer volume"];
    } else if (clickedTab === "Token Holders") {
      return ["input component", "token balance", "transfer volume"];
    } else {
      return ["invalid selection"];
    }
  },
});