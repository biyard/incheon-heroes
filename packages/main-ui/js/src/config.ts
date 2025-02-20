export type Config = {
  canister_id: string;
  endpoint: string;
  klaytn: {
    endpoint: string;
  };
  contracts: {
    shop: string;
  }
};

export const config: Config = {
  canister_id: "",
  endpoint: "",
  klaytn: {
    endpoint: "https://api.baobab.klaytn.net:8651",
  },
  contracts: {
    shop: "",
  }
};
