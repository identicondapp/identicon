#!/bin/bash

./build.sh

export PARENT_ACCT=identicon.testnet
export NEAR_ACCT=identicondapp.identicon.testnet
export VALIDATOR_1=validator1.identicon.testnet
export VALIDATOR_2=validator2.identicon.testnet
export VALIDATOR_3=validator3.identicon.testnet

# delete NEAR_ACCT and send remaining funds to PARENT_ACCT
near delete $NEAR_ACCT $PARENT_ACCT

# create NEAR_ACCT as child of PARENT_ACCT
near create-account $NEAR_ACCT --initialBalance 5 --masterAccount $PARENT_ACCT

# delete validators
near delete $VALIDATOR_1 $PARENT_ACCT
near delete $VALIDATOR_2 $PARENT_ACCT
near delete $VALIDATOR_3 $PARENT_ACCT

# create validators
near create-account $VALIDATOR_1 --initialBalance 5 --masterAccount $PARENT_ACCT
near create-account $VALIDATOR_2 --initialBalance 5 --masterAccount $PARENT_ACCT
near create-account $VALIDATOR_3 --initialBalance 5 --masterAccount $PARENT_ACCT

# deploy contract to NEAR_ACCT
near deploy $NEAR_ACCT --wasmFile res/identicon.wasm --initFunction new --initArgs '{"owner_id": "'$NEAR_ACCT'"}'

# Call some method
# near call $NEAR_ACCT get_solution '{}' --accountId $NEAR_ACCT
