#!/bin/bash

#
# Use:
#   ./deploy.sh PARENT_ACCOUNT_ID CONTRACT_NAME    
#
# Example: 
#   ./deploy.sh identicon.testnet contract_v3
#
export PARENT=$1
export CONTRACT=$2.$PARENT

./build.sh

# we sometimes need to delete the contract, not just overwrite it
# because init can not be called twice on the same contract, and we
# need to cleanup all the data.
# uncomment this line when needed
#near delete $CONTRACT $PARENT

near create-account $CONTRACT --initialBalance 25 --masterAccount $PARENT
near deploy $CONTRACT --wasmFile res/identicon.wasm
