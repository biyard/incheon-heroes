import Caver from "caver-js";
import { shopAbi } from "./shop";

export function initialize_klaytn(endpoint: string, contracts: {
  shop: string,
}): { shop: any } {
  const caver = new Caver(endpoint);
  const shop = caver.contract.create(shopAbi, contracts.shop);

  return { shop };
}

