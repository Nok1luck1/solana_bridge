use alloy::{primitives::address, providers::ProviderBuilder, sol};

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
        function balanceOf(address owner) public view returns (uint256);
        function allowance(address owner,address spender) public view returns(uint256);
        function getOrderInfo(bytes32 orderID) public view returns(Order memory);
   }
}
