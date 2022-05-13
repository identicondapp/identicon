#!/bin/bash
#
# Use:'
#    ./run-request.sh NEAR_ACCOUNT_ID NUMBER 
#
# Example:
#   ./runall.sh yourname.testnet 354
#
# Notes: 
#  - Must call "near login" before running this command.
#  - Number must be a 3 digits number.
#
export PARENT=$1
export NID=$2
export CONTRACT=contract_v3.identicon.testnet
export SUBJECT_ID=ar_dni_12488$NID
export REQUESTOR_ID=$1

# Create a verification request
near call $CONTRACT request_verification '{"requestor_id":"'$REQUESTOR_ID'", "is_type":"ProofOfLife", "subject_id":"'$SUBJECT_ID'", "subject_info":{"age": 65, "sex":"M", "contact":{"phones":"+54-11-6549-4xxx","email": "mazito.v2@gmail.com"},"address":{"directions": "Calle Las Lomitas Nro. 23 e/ Pampa y La Via",   "city": "Adrogue","province": "Buenos Aires","country": "ar","coordinates":{"lat": "","long": ""}}}}' --accountId $PARENT
