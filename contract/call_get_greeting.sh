#!/bin/sh

echo ">> Executing fn get_greeting"

# https://docs.near.org/tools/near-cli#near-dev-deploy
near view near_rust_hello.sunroz.testnet get_greeting

near call near_rust_hello.sunroz.testnet get_predecessor_account_id --accountId sunroz.testnet

near call near_rust_hello.sunroz.testnet get_signer_account_id --accountId sunroz.testnet

near call near_rust_hello.sunroz.testnet get_current_account_id --accountId sunroz.testnet

near call near_rust_hello.sunroz.testnet get_attached_deposit --accountId sunroz.testnet

near call near_rust_hello.sunroz.testnet get_account_balance --accountId sunroz.testnet
