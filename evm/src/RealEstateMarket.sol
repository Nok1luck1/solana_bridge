// SPDX-License-Identifier: UNLICENSED 
pragma solidity 0.8.30;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

contract RealEstateMarket is AccessControl{
    address[] public acceptableTokens;
    constructor(){
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }
    function addAcceptableToken(address token) public onlyRole(DEFAULT_ADMIN_ROLE){
        acceptableTokens.push(token);
    }
    function removeAcceptableToken(address token) public onlyRole(DEFAULT_ADMIN_ROLE){
        for(uint i = 0; i < acceptableTokens.length; i++){
            if(acceptableTokens[i] == token){
                acceptableTokens[i] = acceptableTokens[acceptableTokens.length - 1];
                acceptableTokens.pop();
                break;
            }
        }
    }
    function isAcceptableToken(address token) public view returns(bool){
        for(uint i = 0; i < acceptableTokens.length; i++){
            if(acceptableTokens[i] == token){
                return true;
            }
        }
        return false;
    }

}