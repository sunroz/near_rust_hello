#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deleting previously deployed contract"

near delete near_rust_hello.sunroz.testnet sunroz.testnet

if [ $? -ne 0 ]; then
  echo ">> ERR_DELETE_CONTRACT: Contract does not exist"
  continue;
fi

echo ">> Creating contract account"

near create-account near_rust_hello.sunroz.testnet --masterAccount sunroz.testnet --initialBalance 11

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
near deploy --accountId near_rust_hello.sunroz.testnet --wasmFile ./target/wasm32-unknown-unknown/release/near_rust_hello.wasm
