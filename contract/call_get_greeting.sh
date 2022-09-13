#!/bin/sh

echo ">> Executing fn get_greeting"

# https://docs.near.org/tools/near-cli#near-dev-deploy
near view near_rust_hello.sunroz.testnet get_greeting
