#!/bin/sh

./build.sh

CONFIG=./setting.conf

CONTRACT_ACCOUNT=$(awk '/^CONTRACT_ACCOUNT/{print $3}' "${CONFIG}")
MASTER_ACCOUNT=$(awk '/^MASTER_ACCOUNT/{print $3}' "${CONFIG}")

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deleting previously deployed contract"

near delete $CONTRACT_ACCOUNT $MASTER_ACCOUNT

if [ $? -ne 0 ]; then
  echo ">> ERR_DELETE_CONTRACT: Contract does not exist"
  continue;
fi

echo ">> Creating contract account"

near create-account $CONTRACT_ACCOUNT --masterAccount $MASTER_ACCOUNT --initialBalance 11

echo ">> Deploying contract"

# # https://docs.near.org/tools/near-cli#near-dev-deploy
near deploy --accountId $CONTRACT_ACCOUNT --wasmFile ./target/wasm32-unknown-unknown/release/near_rust_hello.wasm
