const hre = require("hardhat");
const { BigNumber, constants, utils } = require("ethers");


async function main() {
    const [deployer] = await ethers.getSigners();

    const Bridge = await ethers.getContractFactory("Bridge");
    const Token = await ethers.getContractFactory("Token");
    const Factory = await Bridge.deploy(deployer.address);
    await Factory.deployed()
    console.log(Factory.address)
    const Router = await Token.deploy(deployer.address);
    await Router.deployed()
    console.log(Router.address)

}

main()

    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });