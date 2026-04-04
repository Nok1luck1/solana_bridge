const hre = require("hardhat");
const { BigNumber, constants, utils } = require("ethers");


async function main() {
    const [deployer] = await ethers.getSigners();
    const bridgeAddress = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
    const tokenAddress = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512";
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
    const checkOrder = await Bridge.getOrderInfo(orderCreation)
}

main()

    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });