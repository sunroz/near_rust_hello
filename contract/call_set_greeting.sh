#!/bin/sh

CONFIG=./setting.conf

CONTRACT_ACCOUNT=$(awk '/^CONTRACT_ACCOUNT/{print $3}' "${CONFIG}")
MASTER_ACCOUNT=$(awk '/^MASTER_ACCOUNT/{print $3}' "${CONFIG}")

near call $CONTRACT_ACCOUNT get_storage_usage --accountId $MASTER_ACCOUNT

echo ">> Executing fn set_greeting"

near call $CONTRACT_ACCOUNT set_greeting '{"message" : "Learning Rust and Near protocol basics"}' --deposit 5 --accountId $MASTER_ACCOUNT

# ./call_get_greeting.sh
