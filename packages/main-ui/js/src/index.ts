import { Config } from "./config";
import { initialize_klaytn } from "./klaytn/contracts";

async function initialize({
  klaytn,
  contracts,
}: Config) {
  let c = initialize_klaytn(klaytn.endpoint, contracts);
  window.biyard.klaytn = c;
};

declare global {
  interface Window {
    biyard: any;
  }
}

try {
  if (window) {
    window.biyard = {
      initialize,
    };
  }
} catch (e) {
  console.error("inside worker");
}
