#!/bin/bash
#
# Example: 
# ./build.sh mariozito.testnet contract0.identicon.testnet 

export PARENT=$1
export CONTRACT=$2

./build.sh

#near delete $CONTRACT $PARENT
near create-account $CONTRACT --initialBalance 5 --masterAccount $PARENT
near deploy $CONTRACT --wasmFile res/identicon.wasm
