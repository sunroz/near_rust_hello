#!/bin/sh

echo ">> Executing fn set_greeting"

near call near_rust_hello.sunroz.testnet set_greeting '{"message" : "Learning Rust and Near protocol basics"}' --accountId sunroz.testnet

./call_get_greeting.sh
