#!/bin/bash
#
# Example: 
# ./build.sh mariozito.testnet contract0.identicon.testnet 

export PARENT=$1
export CONTRACT=$2.$PARENT

./build.sh

# don't delete the contract, just overwrite it
near delete $CONTRACT $PARENT

near create-account $CONTRACT --initialBalance 25 --masterAccount $PARENT
near deploy $CONTRACT --wasmFile res/identicon.wasm
