// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
import {Bridge} from "../src/Bridge.sol";

contract CounterScript is Script {
    Bridge public counter;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        counter = new Bridge(address(0));

        vm.stopBroadcast();
    }
}
