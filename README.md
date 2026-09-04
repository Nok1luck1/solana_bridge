repo include solidity bridge, solana bridge and rust app that ,ake transfer between networks

to run test anchor keys sync
then start solana-test-validator
then anchor test --skip-local-validator

to use grpc for parsing events from program need https://github.com/rpcpool/yellowstone-grpc pull and build cargo build --release then crreate geyser-config.json inside place where placed solana local validator and set inside file {
  "libpath": "/home/nick/solana/yellowstone-grpc/target/release/libyellowstone_grpc_geyser.so",
  "log": {
    "level": "info"
  },
  "grpc": {
    "address": "0.0.0.0:10000",
    "tls_config": null
  }
} this config with absolute path to file libyellowstone_grpc_geyser.so
then start validator with additional config
solana-test-validator   --geyser-plugin-config geyser-config.json 

////////////////////////////////////
db initialization 

sudo -u bridge_db psql
CREATE DATABASE bridge_db;
CREATE USER admin WITH ENCRYPTED PASSWORD '1111';
GRANT ALL PRIVILEGES ON DATABASE bridge_db TO admin;