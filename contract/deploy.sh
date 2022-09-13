#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
near deploy --accountId near_rust_hello.sunroz.testnet --wasmFile ./target/wasm32-unknown-unknown/release/near_rust_hello.wasm
