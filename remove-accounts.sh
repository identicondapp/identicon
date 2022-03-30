#!/bin/bash

export NEAR_ACCT=identicondapp.identicon.testnet
export PARENT_ACCT=identicon.testnet
export VALIDATOR_1=validator1.identicon.testnet
export VALIDATOR_2=validator2.identicon.testnet
export VALIDATOR_3=validator3.identicon.testnet

# delete NEAR_ACCT and send remaining funds to PARENT_ACCT
near delete $NEAR_ACCT $PARENT_ACCT

# delete validators
near delete $VALIDATOR_1 $PARENT_ACCT
near delete $VALIDATOR_2 $PARENT_ACCT
near delete $VALIDATOR_3 $PARENT_ACCT
