// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;


import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract Token is ERC20{
    constructor (address owner )ERC20("test","tst"){
        _mint(owner,100000000000000000000000);
    }

}