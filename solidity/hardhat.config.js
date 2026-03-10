require("@nomiclabs/hardhat-waffle");
require('hardhat-deploy');
require('@nomiclabs/hardhat-ethers');
require("@nomiclabs/hardhat-etherscan");
require('@openzeppelin/hardhat-upgrades');

const dotenv = require("dotenv");
dotenv.config({ path: __dirname + "/.env" });
const { privateKey } = process.env;


module.exports = {
  defaultNetwork: "hardhat",
  networks: {
    hardhat: {
      chainId: 97, //Binance Smart Chain Testnet
    },
    localhost: {
      url: 'HTTP://127.0.0.1:8545',
    },

  },
  paths: {
    sources: "./contracts",
    artifacts: "./artifacts"
  },
  solidity: {
    compilers: [
      {
        version: "0.8.28",
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        }
      },
      
    ],
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  // solidity: "0.5.16",
};
