const hre = require("hardhat");
const { BigNumber, constants, utils } = require("ethers");


async function main() {
    const [deployer] = await ethers.getSigners();
    const bridgeAddress = "0x0B306BF915C4d645ff596e518fAf3F9669b97016";
    const tokenAddress = "0x959922bE3CAee4b8Cd9a407cc3ac1C251C2007B1";
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