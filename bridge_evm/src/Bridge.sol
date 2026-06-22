// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import {AccessControl} from "openzeppelin-contracts/contracts/access/AccessControl.sol";

//import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
contract Bridge is AccessControl {
    enum OrderType {
        FromEVMtoSol,
        FromEVMtoEVM,
        FomrSolToEVM
    }
    enum StatusOrder {
        Initialized,
        Completed,
        Canceled
    }

    mapping(uint256 => Order) public orderByIndex;
    uint public chain_id;
    uint256 public currentOrderCounter;
    event OrderCreated(uint256 orderId);
    event OrderCanceled(uint256 orderId);
    event OrderExecuted(uint256 orderId);

    struct Order {
        address maker;
        address token0;
        uint amount0;
        uint amount1;
        uint timestamp;
        uint timeexecute;
        string receiver;
        string token1;
        StatusOrder orderStatus;
        OrderType orderType;
    }

    constructor(address owner) {
        _grantRole(DEFAULT_ADMIN_ROLE, owner);
        chain_id = block.chainid;
    }

    function order_for_transfer_sol(
        address token0,
        uint amount0,
        uint amount1,
        string memory solAddress,
        string memory solMintAcc
    ) public returns (uint256 orderId) {
        Order memory order = Order({
            maker: msg.sender,
            token0: token0,
            amount0: amount0,
            amount1: amount1,
            timestamp: block.timestamp,
            timeexecute: 0,
            receiver: solAddress,
            token1: solMintAcc,
            orderStatus: StatusOrder.Initialized,
            orderType: OrderType.FromEVMtoSol
        });
        orderId = currentOrderCounter;
        currentOrderCounter++;
        IERC20(token0).transfer(address(this), amount0);
        orderByIndex[orderId] = order;
        emit OrderCreated(orderId);
    }

    function order_for_transfer_evm(
        address token0,
        uint amount0,
        uint amount1,
        string memory user_destination_address,
        string memory token_address
    ) public returns (uint256 orderId) {
        Order memory order = Order({
            maker: msg.sender,
            token0: token0,
            amount0: amount0,
            amount1: amount1,
            timestamp: block.timestamp,
            timeexecute: 0,
            receiver: user_destination_address,
            token1: token_address,
            orderStatus: StatusOrder.Initialized,
            orderType: OrderType.FromEVMtoEVM
        });
        orderId = currentOrderCounter;
        currentOrderCounter++;
        IERC20(token0).transfer(address(this), amount0);
        orderByIndex[orderId] = order;
        emit OrderCreated(orderId);
    }

    function order_for_execution(
        uint256 timeinited,
        address receiver,
        string memory _token0,
        address token1,
        string memory sender,
        uint256 amount0,
        uint256 amount1
    ) public onlyRole(DEFAULT_ADMIN_ROLE) returns (uint256 orderId) {
        uint256 balanceTokenForReward = IERC20(token1).balanceOf(address(this));
        require(
            balanceTokenForReward >= amount1,
            "Inssuficient amount to distribute reward"
        );
        Order memory order = Order({
            maker: msg.sender,
            token0: token1,
            amount0: amount0,
            amount1: amount1,
            timestamp: timeinited,
            timeexecute: block.timestamp,
            receiver: sender,
            token1: _token0,
            orderStatus: StatusOrder.Completed,
            orderType: OrderType.FromEVMtoSol
        });
        orderId = currentOrderCounter;
        currentOrderCounter++;
        IERC20(token1).transferFrom(address(this), receiver, amount1);
        orderByIndex[orderId] = order;
        emit OrderExecuted(orderId);
    }

    function cancel_order(uint order_id) public {
        Order memory order = orderByIndex[order_id];
        require(order.orderStatus == StatusOrder.Initialized);
        require(order.maker == msg.sender);
        IERC20(order.token0).transferFrom(
            address(this),
            order.maker,
            order.amount0
        );
        emit OrderCanceled(order_id);
    }

    function getOrderInfo(uint256 orderID) public view returns (Order memory) {
        return orderByIndex[orderID];
    }
}
