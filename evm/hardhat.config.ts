import { defineConfig } from "hardhat/config";
import hardhatEthers from "@nomicfoundation/hardhat-ethers";

export default defineConfig({
    plugins: [hardhatEthers],

    solidity: {
        version: "0.8.30",
        settings: {
            optimizer: {
                enabled: true,
                runs: 200,
            },
        },
    },

    paths: {
        sources: "./src",
        tests: "./test/hardhat",
        cache: "./cache",
        artifacts: "./artifacts",
    },
});