#!/bin/bash

if [[ $# -eq 0 ]] ; then
    echo 'Please, call this script as: ./runall.sh PARENT_ACCOUNT_ID ( eg: ./runall.sh yourname.testnet ).'
    echo 'Note: you must call "near login" before running this command.'
    exit 0
fi

export PARENT=$1
export CONTRACT=contract_v3.$PARENT
export REQUESTOR_ID=maz.testnet

#./deploy.sh $PARENT $CONTRACT

# 0. Delete / Create testing subaccounts for validators
# near state validator01.$PARENT && near delete validator01.$PARENT $PARENT
# near state validator02.$PARENT && near delete validator02.$PARENT $PARENT
# near state validator03.$PARENT && near delete validator03.$PARENT $PARENT
# near state validator04.$PARENT && near delete validator04.$PARENT $PARENT
# near state validator05.$PARENT && near delete validator05.$PARENT $PARENT
# near state validator06.$PARENT && near delete validator06.$PARENT $PARENT
# near state validator07.$PARENT && near delete validator07.$PARENT $PARENT
# near state validator08.$PARENT && near delete validator08.$PARENT $PARENT
# 
# near create-account validator01.$PARENT --initialBalance 5 --masterAccount $PARENT
# near create-account validator02.$PARENT --initialBalance 5 --masterAccount $PARENT
# near create-account validator03.$PARENT --initialBalance 5 --masterAccount $PARENT
# near create-account validator04.$PARENT --initialBalance 5 --masterAccount $PARENT
# near create-account validator05.$PARENT --initialBalance 5 --masterAccount $PARENT
# near create-account validator06.$PARENT --initialBalance 5 --masterAccount $PARENT
# near create-account validator07.$PARENT --initialBalance 5 --masterAccount $PARENT
# near create-account validator08.$PARENT --initialBalance 5 --masterAccount $PARENT

# 1. Initialize contract
near call $CONTRACT new --accountId $PARENT

# 2. Register some validators 
near call $CONTRACT register_as_validator '{"validator_id":"validator01.'$PARENT'"}'  --accountId $PARENT
near call $CONTRACT register_as_validator '{"validator_id":"validator02.'$PARENT'"}'  --accountId $PARENT
near call $CONTRACT register_as_validator '{"validator_id":"validator03.'$PARENT'"}'  --accountId $PARENT
near call $CONTRACT register_as_validator '{"validator_id":"validator04.'$PARENT'"}'  --accountId $PARENT
near call $CONTRACT register_as_validator '{"validator_id":"validator05.'$PARENT'"}'  --accountId $PARENT
near call $CONTRACT register_as_validator '{"validator_id":"validator06.'$PARENT'"}'  --accountId $PARENT
near call $CONTRACT register_as_validator '{"validator_id":"validator07.'$PARENT'"}'  --accountId $PARENT
near call $CONTRACT register_as_validator '{"validator_id":"validator08.'$PARENT'"}'  --accountId $PARENT

# 3. Create a verification request
near call $CONTRACT request_verification '{"requestor_id":"'$REQUESTOR_ID'", "is_type":"ProofOfLife", "subject_id":"ar_dni_12488353", "subject_info":{"age": 65, "sex":"M", "contact":{"phones":"+54-11-6549-4xxx","email": "mazito.v2@gmail.com"},"address":{"directions": "Calle Las Lomitas Nro. 23 e/ Pampa y La Via",   "city": "Adrogue","province": "Buenos Aires","country": "ar","coordinates":{"lat": "","long": ""}}}}' --accountId $PARENT

# 4. Send verification results by validators
near call $CONTRACT report_verification_result '{"validator_id":"validator02.'$PARENT'", "subject_id":"ar_dni_12488353", "stated":"Approved", "cause":""}' --accountId $PARENT
near call $CONTRACT report_verification_result '{"validator_id":"validator03.'$PARENT'", "subject_id":"ar_dni_12488353", "stated":"Rejected", "cause":""}' --accountId $PARENT
near call $CONTRACT report_verification_result '{"validator_id":"validator04.'$PARENT'", "subject_id":"ar_dni_12488353", "stated":"Approved", "cause":""}' --accountId $PARENT

# 5. Pay for approved or rejected validations
near call $CONTRACT pay_validators '{"requestor_id":"'$REQUESTOR_ID'", "subject_id":"ar_dni_12488353"}' --accountId $PARENT
