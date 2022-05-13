#!/bin/bash
#
# Use:'
#    ./run-bindcard.sh NEAR_ACCOUNT_ID NUMBER 
#
# Example:
#   ./run-bindcard.sh yourname.testnet 354
#
# Notes: 
#  - Must call "near login" before running this command.
#  - Number must be a 3 digits number.
#

export PARENT=$1
export NID=$2
export CONTRACT=contract_v3.identicon.testnet
export SUBJECT_ID=ar_dni_12488$NID
export CARD_CID=Qmc3kQzgwWof7mLG7PPPb1vx6DYDqv9YrW1nWBGhHRqiWW

# Create a verification request
near call $CONTRACT bind_card_file '{"subject_id":"'$SUBJECT_ID'", "card_id":"'$CARD_CID'"}' --accountId $PARENT
