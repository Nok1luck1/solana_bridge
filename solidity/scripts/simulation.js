const hre = require("hardhat");
const { BigNumber, constants, utils } = require("ethers");


async function main() {
    const [deployer] = await ethers.getSigners();
    const bridgeAddress = "0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9";
    const tokenAddress = "0x5FC8d32690cc91D4c39d9d3abcBD16989F875707";
    const amount0 = "10000000000000";
    const amount1 = "10000000000000";
    const solreceiver = "8nkxdKGDpJYTvrS2UoQh4QrGS3FLmHRCdhWtriGcFFB";
    const tokenmintsol = "8nkxdKGDpJYTvrS2UoQh4QrGS3FLmHRCdhWtriGcFFB";
    const Bridge = await ethers.getContractAt("Bridge",bridgeAddress);
    const Tokencontr = await ethers.getContractAt("Token",tokenAddress);
    const approve = await Tokencontr.approve(Bridge.address,amount0 );
    await approve.wait();
    const orderCreation = await Bridge.order_for_transfer(Tokencontr.address,amount0,amount1,solreceiver,tokenmintsol)
    console.log(orderCreation,"order creation");
}

main()

    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });