use alloy::sol;

sol! {
   // The `rpc` attribute enables contract interaction via the provider.
   #[sol(rpc)]
   contract ERC20 {
        function balanceOf(address owner) public view returns (uint256);
        function allowance(address owner,address spender) public view returns(uint256);


   }
   #[sol(rpc)]
   contract Bridge {
     enum OrderType {
        FromEVMtoSol,
        FomrSolToEVM
    }
    enum StatusOrder {
        Initialized,
        Completed,
        Canceled
    }
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
        event OrderCreated(bytes32 orderId);
        event OrderExecuted(bytes32 orderId);
        function balanceOf(address owner) public view returns (uint256);
        function allowance(address owner,address spender) public view returns(uint256);
        function getOrderInfo(bytes32 orderID) public view returns(Order memory);
        function distributeReward(address receiver,string memory _token0,address token1,string memory sender,uint256 amount0,uint256 amount1) public returns (bytes32 orderId);
   }
}
