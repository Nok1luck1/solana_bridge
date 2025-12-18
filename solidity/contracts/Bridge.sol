// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

// Uncomment this line to use console.log
// import "hardhat/console.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract Bridge {
  enum OrderType{
    FromEVMtoSol,
    FomrSolToEVM
  }
  enum StatusOrder{
    Initialized,
    Completed,
    Canceled
  }
  address public owner;
  mapping (bytes32 =>Order) public orderByIndex;
  event OrderCreated(bytes32 orderId);

  struct Order{
    address user;
    address token0;
    uint amount0;
    uint amount1;
    uint timestamp;
    string userSol;
    string token1;
    StatusOrder orderStatus;
    OrderType orderType;
  }

  constructor()  {  
    owner = msg.sender;
  }
  function sendToSol(address token0,uint amountGive,uint amountExpected,string memory solAddress,string memory solMintAcc) public {
    Order memory order = Order ({
      user:msg.sender,
      token0: token0,
      amount0:amountGive,
      amount1:amountExpected,
      timestamp: block.timestamp,
      userSol:solAddress,
      token1:solMintAcc,
      orderStatus:StatusOrder.Initialized,
      orderType:OrderType.FromEVMtoSol
    });
    bytes32 orderId =  keccak256(abi.encodePacked(order.user,order.timestamp,order.token0,order.token1));
    IERC20(token0).transfer(address(this),amountGive);
    orderByIndex[orderId] = order;
    emit OrderCreated(orderId);
  }

  
}
 