#!/bin/sh

CONFIG=./setting.conf

CONTRACT_ACCOUNT=$(awk '/^CONTRACT_ACCOUNT/{print $3}' "${CONFIG}")
MASTER_ACCOUNT=$(awk '/^MASTER_ACCOUNT/{print $3}' "${CONFIG}")

echo ">> Executing fn set_greeting"

near call $CONTRACT_ACCOUNT set_greeting '{"message" : "Learning Rust and Near protocol basics"}' --accountId $MASTER_ACCOUNT

./call_get_greeting.sh
