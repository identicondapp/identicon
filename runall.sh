#!/bin/bash
export PARENT=identicon.testnet
export CONTRACT=contract0.$PARENT

sh -v deploy.sh

# 1. Initialize contract
near call $CONTRACT new --accountId identicon.testnet

# 2. Regsiter some validators 
near call $CONTRACT register_as_validator '{"validator_id":"validator01.identicon.testnet"}'  --accountId identicon.testnet
near call $CONTRACT register_as_validator '{"validator_id":"validator02.identicon.testnet"}'  --accountId identicon.testnet
near call $CONTRACT register_as_validator '{"validator_id":"validator03.identicon.testnet"}'  --accountId identicon.testnet
near call $CONTRACT register_as_validator '{"validator_id":"validator04.identicon.testnet"}'  --accountId identicon.testnet
near call $CONTRACT register_as_validator '{"validator_id":"validator05.identicon.testnet"}'  --accountId identicon.testnet
near call $CONTRACT register_as_validator '{"validator_id":"validator06.identicon.testnet"}'  --accountId identicon.testnet
near call $CONTRACT register_as_validator '{"validator_id":"validator07.identicon.testnet"}'  --accountId identicon.testnet
near call $CONTRACT register_as_validator '{"validator_id":"validator08.identicon.testnet"}'  --accountId identicon.testnet

# 3. Create a verification request
near call $CONTRACT request_verification '{"requestor_id":"juanmescher.testnet", "is_type":"ProofOfLife", "subject_id":"ar_dni_12488353", "subject_info":{"age": 65, "sex":"M", "contact":{"phones":"+54-11-6549-4xxx","email": "mazito.v2@gmail.com"},"address":{"directions": "Calle Las Lomitas Nro. 23 e/ Pampa y La Via",   "city": "Adrogue","province": "Buenos Aires","country": "ar","coordinates":{"lat": "","long": ""}}}}' --accountId identicon.testnet

# 4. Send verification results by validators
near call $CONTRACT report_verification_result '{"validator_id":"validator02.identicon.testnet", "subject_id":"ar_dni_12488353", "stated":"Approved", "cause":""}' --accountId identicon.testnet
near call $CONTRACT report_verification_result '{"validator_id":"validator03.identicon.testnet", "subject_id":"ar_dni_12488353", "stated":"Rejected", "cause":""}' --accountId identicon.testnet
near call $CONTRACT report_verification_result '{"validator_id":"validator04.identicon.testnet", "subject_id":"ar_dni_12488353", "stated":"Approved", "cause":""}' --accountId identicon.testnet
