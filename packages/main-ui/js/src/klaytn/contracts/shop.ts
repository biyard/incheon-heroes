export const shopAbi: any = [
  { "inputs": [], "stateMutability": "nonpayable", "type": "constructor" },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "address",
        "name": "sender",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "user",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "supply",
        "type": "uint256"
      }
    ],
    "name": "AddWhitelist",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "address",
        "name": "sender",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "timestamp",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "itemId",
        "type": "uint256"
      }
    ],
    "name": "BuyItem",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "address",
        "name": "sender",
        "type": "address"
      },
      {
        "components": [
          { "internalType": "uint256", "name": "id", "type": "uint256" },
          { "internalType": "uint256", "name": "price", "type": "uint256" },
          { "internalType": "uint256", "name": "tokenId", "type": "uint256" },
          { "internalType": "string", "name": "name", "type": "string" },
          { "internalType": "string", "name": "image", "type": "string" },
          { "internalType": "uint256", "name": "supply", "type": "uint256" },
          { "internalType": "uint256", "name": "likes", "type": "uint256" },
          { "internalType": "uint256", "name": "reports", "type": "uint256" },
          {
            "internalType": "address",
            "name": "contractAddress",
            "type": "address"
          },
          { "internalType": "address", "name": "creator", "type": "address" },
          {
            "internalType": "uint256",
            "name": "remaining",
            "type": "uint256"
          },
          { "internalType": "uint8", "name": "level", "type": "uint8" },
          { "internalType": "string", "name": "metadata", "type": "string" }
        ],
        "indexed": false,
        "internalType": "struct Item[]",
        "name": "items",
        "type": "tuple[]"
      }
    ],
    "name": "CreateShopItems",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "address",
        "name": "sender",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "itemId",
        "type": "uint256"
      }
    ],
    "name": "DeleteItem",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "address",
        "name": "sender",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "itemId",
        "type": "uint256"
      }
    ],
    "name": "LikeItem",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "address",
        "name": "sender",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "itemId",
        "type": "uint256"
      }
    ],
    "name": "ReportItem",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "address",
        "name": "sender",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "account",
        "type": "address"
      }
    ],
    "name": "SetContractAddress",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "address",
        "name": "sender",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "itemId",
        "type": "uint256"
      },
      {
        "components": [
          { "internalType": "uint256", "name": "id", "type": "uint256" },
          { "internalType": "uint256", "name": "price", "type": "uint256" },
          { "internalType": "uint256", "name": "tokenId", "type": "uint256" },
          { "internalType": "string", "name": "name", "type": "string" },
          { "internalType": "string", "name": "image", "type": "string" },
          { "internalType": "uint256", "name": "supply", "type": "uint256" },
          { "internalType": "uint256", "name": "likes", "type": "uint256" },
          { "internalType": "uint256", "name": "reports", "type": "uint256" },
          {
            "internalType": "address",
            "name": "contractAddress",
            "type": "address"
          },
          { "internalType": "address", "name": "creator", "type": "address" },
          {
            "internalType": "uint256",
            "name": "remaining",
            "type": "uint256"
          },
          { "internalType": "uint8", "name": "level", "type": "uint8" },
          { "internalType": "string", "name": "metadata", "type": "string" }
        ],
        "indexed": false,
        "internalType": "struct Item",
        "name": "item",
        "type": "tuple"
      }
    ],
    "name": "UpdateItem",
    "type": "event"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "userAddress", "type": "address" },
      { "internalType": "uint256", "name": "supply", "type": "uint256" },
      { "internalType": "uint256[]", "name": "itemIds", "type": "uint256[]" }
    ],
    "name": "addWhitelist",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "itemId", "type": "uint256" }
    ],
    "name": "alreadyLike",
    "outputs": [
      { "internalType": "bool", "name": "", "type": "bool" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "itemId", "type": "uint256" }
    ],
    "name": "alreadyReport",
    "outputs": [
      { "internalType": "bool", "name": "", "type": "bool" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "itemId", "type": "uint256" }
    ],
    "name": "buyItem",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "components": [
          { "internalType": "uint256", "name": "id", "type": "uint256" },
          { "internalType": "uint256", "name": "price", "type": "uint256" },
          { "internalType": "uint256", "name": "tokenId", "type": "uint256" },
          { "internalType": "string", "name": "name", "type": "string" },
          { "internalType": "string", "name": "image", "type": "string" },
          { "internalType": "uint256", "name": "supply", "type": "uint256" },
          { "internalType": "uint256", "name": "likes", "type": "uint256" },
          { "internalType": "uint256", "name": "reports", "type": "uint256" },
          {
            "internalType": "address",
            "name": "contractAddress",
            "type": "address"
          },
          { "internalType": "address", "name": "creator", "type": "address" },
          {
            "internalType": "uint256",
            "name": "remaining",
            "type": "uint256"
          },
          { "internalType": "uint8", "name": "level", "type": "uint8" },
          { "internalType": "string", "name": "metadata", "type": "string" }
        ],
        "internalType": "struct Item[]",
        "name": "items",
        "type": "tuple[]"
      }
    ],
    "name": "createShopItems",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "addr", "type": "address" }
    ],
    "name": "delTrustedParty",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "itemId", "type": "uint256" }
    ],
    "name": "deleteItem",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes",
        "name": "functionSignature",
        "type": "bytes"
      }
    ],
    "name": "executeCode",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "forwarder",
    "outputs": [
      { "internalType": "address", "name": "", "type": "address" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "itemId", "type": "uint256" }
    ],
    "name": "getShopItem",
    "outputs": [
      {
        "components": [
          { "internalType": "uint256", "name": "id", "type": "uint256" },
          { "internalType": "uint256", "name": "price", "type": "uint256" },
          { "internalType": "uint256", "name": "tokenId", "type": "uint256" },
          { "internalType": "string", "name": "name", "type": "string" },
          { "internalType": "string", "name": "image", "type": "string" },
          { "internalType": "uint256", "name": "supply", "type": "uint256" },
          { "internalType": "uint256", "name": "likes", "type": "uint256" },
          { "internalType": "uint256", "name": "reports", "type": "uint256" },
          {
            "internalType": "address",
            "name": "contractAddress",
            "type": "address"
          },
          { "internalType": "address", "name": "creator", "type": "address" },
          {
            "internalType": "uint256",
            "name": "remaining",
            "type": "uint256"
          },
          { "internalType": "uint8", "name": "level", "type": "uint8" },
          { "internalType": "string", "name": "metadata", "type": "string" }
        ],
        "internalType": "struct Item",
        "name": "",
        "type": "tuple"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "userAddress", "type": "address" },
      { "internalType": "uint256", "name": "tokenId", "type": "uint256" }
    ],
    "name": "isMinted",
    "outputs": [
      { "internalType": "bool", "name": "", "type": "bool" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "addr", "type": "address" }
    ],
    "name": "isTrusted",
    "outputs": [
      { "internalType": "bool", "name": "", "type": "bool" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "userAddress", "type": "address" }
    ],
    "name": "isWhitelisted",
    "outputs": [
      { "internalType": "bool", "name": "", "type": "bool" },
      { "internalType": "uint256", "name": "", "type": "uint256" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "itemId", "type": "uint256" }
    ],
    "name": "likeItem",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "page", "type": "uint256" },
      { "internalType": "uint256", "name": "size", "type": "uint256" }
    ],
    "name": "listItems",
    "outputs": [
      {
        "components": [
          { "internalType": "uint256", "name": "id", "type": "uint256" },
          { "internalType": "uint256", "name": "price", "type": "uint256" },
          { "internalType": "uint256", "name": "tokenId", "type": "uint256" },
          { "internalType": "string", "name": "name", "type": "string" },
          { "internalType": "string", "name": "image", "type": "string" },
          { "internalType": "uint256", "name": "supply", "type": "uint256" },
          { "internalType": "uint256", "name": "likes", "type": "uint256" },
          { "internalType": "uint256", "name": "reports", "type": "uint256" },
          {
            "internalType": "address",
            "name": "contractAddress",
            "type": "address"
          },
          { "internalType": "address", "name": "creator", "type": "address" },
          {
            "internalType": "uint256",
            "name": "remaining",
            "type": "uint256"
          },
          { "internalType": "uint8", "name": "level", "type": "uint8" },
          { "internalType": "string", "name": "metadata", "type": "string" }
        ],
        "internalType": "struct Item[]",
        "name": "",
        "type": "tuple[]"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "itemId", "type": "uint256" }
    ],
    "name": "listLikersByItemId",
    "outputs": [
      { "internalType": "address[]", "name": "", "type": "address[]" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "addr", "type": "address" }
    ],
    "name": "listLikesByAddress",
    "outputs": [
      { "internalType": "uint256[]", "name": "", "type": "uint256[]" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "itemId", "type": "uint256" }
    ],
    "name": "listReportersByItemId",
    "outputs": [
      { "internalType": "address[]", "name": "", "type": "address[]" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "listTurstedParties",
    "outputs": [
      { "internalType": "address[]", "name": "", "type": "address[]" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "nextIdOf",
    "outputs": [
      { "internalType": "uint256", "name": "", "type": "uint256" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "owner",
    "outputs": [
      { "internalType": "address", "name": "", "type": "address" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "itemId", "type": "uint256" }
    ],
    "name": "reportItem",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "accountExperience",
        "type": "address"
      }
    ],
    "name": "setContractAddress",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "forwarder", "type": "address" }
    ],
    "name": "setForwarder",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "addr", "type": "address" }
    ],
    "name": "setTrustedParty",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "itemId", "type": "uint256" },
      {
        "components": [
          { "internalType": "uint256", "name": "id", "type": "uint256" },
          { "internalType": "uint256", "name": "price", "type": "uint256" },
          { "internalType": "uint256", "name": "tokenId", "type": "uint256" },
          { "internalType": "string", "name": "name", "type": "string" },
          { "internalType": "string", "name": "image", "type": "string" },
          { "internalType": "uint256", "name": "supply", "type": "uint256" },
          { "internalType": "uint256", "name": "likes", "type": "uint256" },
          { "internalType": "uint256", "name": "reports", "type": "uint256" },
          {
            "internalType": "address",
            "name": "contractAddress",
            "type": "address"
          },
          { "internalType": "address", "name": "creator", "type": "address" },
          {
            "internalType": "uint256",
            "name": "remaining",
            "type": "uint256"
          },
          { "internalType": "uint8", "name": "level", "type": "uint8" },
          { "internalType": "string", "name": "metadata", "type": "string" }
        ],
        "internalType": "struct Item",
        "name": "item",
        "type": "tuple"
      }
    ],
    "name": "updateItem",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  }
];
