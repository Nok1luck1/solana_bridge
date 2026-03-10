const hre = require("hardhat");
const { BigNumber, constants, utils } = require("ethers");


async function main() {
    const [deployer] = await ethers.getSigners();

    const Bridge = await ethers.getContractFactory("Bridge");
    const Tokencontr = await ethers.getContractFactory("Token");
    const BridgeContract = await Bridge.deploy(deployer.address);
    await BridgeContract.deployed()
    console.log(BridgeContract.address,"bridge contract")
    const Token = await Tokencontr.deploy(deployer.address);
    await Token.deployed()
    console.log(Token.address,"token Contractr")

}

main()

    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });