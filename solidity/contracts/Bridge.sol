// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

// Uncomment this line to use console.log
// import "hardhat/console.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

contract Bridge is AccessControl {
    enum OrderType {
        FromEVMtoSol,
        FomrSolToEVM
    }
    enum StatusOrder {
        Initialized,
        Completed,
        Canceled
    }
    mapping(bytes32 => Order) public orderByIndex;
    event OrderCreated(bytes32 orderId);
    event OrderExecuted(bytes32 orderId);

    struct Order {
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
    constructor(address owner) {
        _grantRole(DEFAULT_ADMIN_ROLE, owner);
    }
    function sendToSol(
        address token0,
        uint amountGive,
        uint amountExpected,
        string memory solAddress,
        string memory solMintAcc
    ) public {
        Order memory order = Order({
            user: msg.sender,
            token0: token0,
            amount0: amountGive,
            amount1: amountExpected,
            timestamp: block.timestamp,
            userSol: solAddress,
            token1: solMintAcc,
            orderStatus: StatusOrder.Initialized,
            orderType: OrderType.FromEVMtoSol
        });
        bytes32 orderId = keccak256(
            abi.encodePacked(
                order.user,
                order.timestamp,
                order.token0,
                order.token1
            )
        );
        IERC20(token0).transfer(address(this), amountGive);
        orderByIndex[orderId] = order;
        emit OrderCreated(orderId);
    }
    function distributeReward(
        address receiver,
        string memory _token0,
        address token1,
        string memory sender,
        uint256 amount0,
        uint256 amount1
    ) public onlyRole(DEFAULT_ADMIN_ROLE) returns (bytes32 orderId) {
        uint256 balanceTokenForReward = IERC20(token1).balanceOf(address(this));
        require(
            balanceTokenForReward >= amount1,
            "Inssuficient amount to distribute reward"
        );
        Order memory order = Order({
            user: msg.sender,
            token0: token1,
            amount0: amount0,
            amount1: amount1,
            timestamp: block.timestamp,
            userSol: sender,
            token1: _token0,
            orderStatus: StatusOrder.Initialized,
            orderType: OrderType.FromEVMtoSol
        });
        orderId = keccak256(
            abi.encodePacked(
                order.user,
                order.timestamp,
                order.token0,
                order.token1
            )
        );
        IERC20(token1).transferFrom(address(this), receiver, amount1);
        orderByIndex[orderId] = order;
        emit OrderExecuted(orderId);
    }
    function getOrderInfo(bytes32 orderID) public view returns (Order memory) {
        return orderByIndex[orderID];
    }
}
