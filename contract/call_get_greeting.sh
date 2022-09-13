#!/bin/sh

CONFIG=./setting.conf

CONTRACT_ACCOUNT=$(awk '/^CONTRACT_ACCOUNT/{print $3}' "${CONFIG}")
MASTER_ACCOUNT=$(awk '/^MASTER_ACCOUNT/{print $3}' "${CONFIG}")

echo ">> Executing fn get_greeting"

# https://docs.near.org/tools/near-cli#near-dev-deploy
near view $CONTRACT_ACCOUNT get_greeting '{}'

near call $CONTRACT_ACCOUNT get_predecessor_account_id --accountId $MASTER_ACCOUNT

near call $CONTRACT_ACCOUNT get_signer_account_id --accountId $MASTER_ACCOUNT

near call $CONTRACT_ACCOUNT get_current_account_id --accountId $MASTER_ACCOUNT

near call $CONTRACT_ACCOUNT get_attached_deposit --accountId $MASTER_ACCOUNT

near call $CONTRACT_ACCOUNT get_attached_deposit --accountId $MASTER_ACCOUNT --deposit 2

near call $CONTRACT_ACCOUNT get_account_balance --accountId $MASTER_ACCOUNT

near call $CONTRACT_ACCOUNT get_gas_info --accountId $MASTER_ACCOUNT

near call $CONTRACT_ACCOUNT get_storage_usage --accountId $MASTER_ACCOUNT

near call $CONTRACT_ACCOUNT get_storage_byte_cost --accountId $MASTER_ACCOUNT

near call $CONTRACT_ACCOUNT get_total_storage_cost --accountId $MASTER_ACCOUNT
