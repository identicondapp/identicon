
# Actividades Día 2 (Miercoles 11-May)

### Inicial

Agregar `cards` a `VerificationContract` y mantener la version anterior como `VerificationContractV1`
~~~rust
    // emmited certification cards for approved subjects
    pub cards: UnorderedMap<SubjectId, FileId>
~~~

Agregar el codigo de migración al contrato:
~~~rust
#[near_bindgen]
impl VerificationContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        log!("\ninit:: initialized contract state v2: verifications, assignments, validators, cards");
        Self {
            // this are Contract v1 props
            verifications: UnorderedMap::new(b"c"),
            assignments: UnorderedMap::new(b"u"),
            validators: Vec::new(),

            // this is the new addition to Contract V2
            cards: UnorderedMap::new(b"v"), 
        }
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        log!("\nmigrate: migrating contract state v1 to v2: verifications, assignments, validators");
        let old_state: VerificationContractV1 = env::state_read().expect("failed");
        Self {
            // this props values must be preserved
            verifications: old_state.verifications,
            assignments: old_state.assignments,
            validators: old_state.validators,

            // now initialize the new 'cards' map
            cards: UnorderedMap::new(b"v"), 
        }
    }
~~~

Correr tests unitarios para verificar que sigue andando todo - OK

### Testing de la migración

Antes de testear la migración:

- agregar cambios al branch actual, la version V2 del contrato `ncar/maz`
- crear un branch con la version V1 del contrato `ncar/maz/contractv1`
- reset de ese branch al commit previo a los cambios y dejar así

Para testear la migración de forma repetitiva: 

1. checkout del branch V1 => tenemos el codigo del contrato V1 
2. build, deploy del contrato
3. inicializamos, y corremos `runall.sh` para cargarle datos => ya tenemos el estado del contrato con la estructura de V1
4. checkout del branch modificado y build =>  tenemos el codigo del contrato V2
5. deploy del contrato modificado con `near deploy ... --initFunction "migrate" ...`

**Pasos 1 y 2**
~~~shell
mzito@mariodesk:~/dev/learn/near/NCAR-Bootcamp-05-2022$ ./deploy.sh identicon.testnet contract_v1.identicon.testnet
   Compiling identicon v0.2.0 (/home/mzito/dev/learn/near/NCAR-Bootcamp-05-2022)
    Finished release [optimized] target(s) in 3.97s
Saving key to '/home/mzito/.near-credentials/testnet/contract_v1.identicon.testnet.json'
Account contract_v1.identicon.testnet for network "testnet" was created.
Starting deployment. Account id: contract_v1.identicon.testnet, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: res/identicon.wasm
Transaction Id CGjL5fW3uKfK9j5eBqXupoedPJNBy9oWDnKBgBzfUKEo
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/CGjL5fW3uKfK9j5eBqXupoedPJNBy9oWDnKBgBzfUKEo
Done deploying to contract_v1.identicon.testnet
~~~

**Paso 3** 
~~~shell
mzito@mariodesk:~/dev/learn/near/NCAR-Bootcamp-05-2022$ ./runall.sh identicon.testnet
---
Scheduling a call: contract_v1.identicon.testnet.new()
Doing account.functionCall()
Receipt: 9y8rwwzpjrz4Z2h8ixCTe8JKmhcNvjDCD1X4MkdUf4ph
        Log [contract_v1.identicon.testnet]: 
initialized contract state: [verifications], [assignments], [validators]
Transaction Id HHWaiZJqeNxXzCM9JGev8kqyiBq8f9sasFLdJCMYZkXG
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/HHWaiZJqeNxXzCM9JGev8kqyiBq8f9sasFLdJCMYZkXG
''
---
Scheduling a call: contract_v1.identicon.testnet.register_as_validator({"validator_id":"validator01.identicon.testnet"})
Doing account.functionCall()
Receipt: GMWNuWQvJCE88ixxaQJ8gnku1jM9QG1ycoVnC2veDePL
        Log [contract_v1.identicon.testnet]: "validator01.identicon.testnet"
Transaction Id 3KvR3Ns7znnFqxwPBhfekSQmbio7QnKkazrj6Kzc4X5Q
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/3KvR3Ns7znnFqxwPBhfekSQmbio7QnKkazrj6Kzc4X5Q
''
---
Scheduling a call: contract_v1.identicon.testnet.register_as_validator({"validator_id":"validator02.identicon.testnet"})
Doing account.functionCall()
Receipt: GSFECUGmex7hrsSoj6sZP6C2r2Np5cDmo9hbgXuW9PM1
        Log [contract_v1.identicon.testnet]: "validator02.identicon.testnet"
Transaction Id W1TThvaT9W9XNTkZhKyeFPnqcmGtk8sx3VF9JvJBibr
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/W1TThvaT9W9XNTkZhKyeFPnqcmGtk8sx3VF9JvJBibr
''
---
Scheduling a call: contract_v1.identicon.testnet.register_as_validator({"validator_id":"validator03.identicon.testnet"})
Doing account.functionCall()
Receipt: AVo2R3m6sQ5DjcPrg2FCUy2Wxc2xR8YeK7Kepn1J9adC
        Log [contract_v1.identicon.testnet]: "validator03.identicon.testnet"
Transaction Id G179NGmJpymPKGT5G61Y26ktUn9nxxE8sh3uHmQ4Ap6s
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/G179NGmJpymPKGT5G61Y26ktUn9nxxE8sh3uHmQ4Ap6s
''
Scheduling a call: contract_v1.identicon.testnet.register_as_validator({"validator_id":"validator04.identicon.testnet"})
Doing account.functionCall()
Receipt: BBvpKQSaZLHXWkMv22DYAnX8KyHsisZQfmdecYU8Fd7v
        Log [contract_v1.identicon.testnet]: "validator04.identicon.testnet"
Transaction Id GTQuQpwcFriyCiZq5xxmRHLzKFCgu6CwH9wi9bkHJtnx
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/GTQuQpwcFriyCiZq5xxmRHLzKFCgu6CwH9wi9bkHJtnx
''
---
Scheduling a call: contract_v1.identicon.testnet.register_as_validator({"validator_id":"validator05.identicon.testnet"})
Doing account.functionCall()
Receipt: 9cpL1YwoL2uEtoWe2eqvybg1daz2MmKeBCf4UL3pMejJ
        Log [contract_v1.identicon.testnet]: "validator05.identicon.testnet"
Transaction Id FPMVh6ZH1FPeUtvh7r4DweaZyQxj3ZjApdVquz9ZdDo7
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/FPMVh6ZH1FPeUtvh7r4DweaZyQxj3ZjApdVquz9ZdDo7
''
---
Scheduling a call: contract_v1.identicon.testnet.register_as_validator({"validator_id":"validator06.identicon.testnet"})
Doing account.functionCall()
Receipt: H85hiEQprv3cFyo5R2ePKbbpR8ADezBiPW1aD6YtD49N
        Log [contract_v1.identicon.testnet]: "validator06.identicon.testnet"
Transaction Id EdMhkYZykxUUuNFEA5HtMYhWL2JXzJqwM5tr6w2pXytL
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/EdMhkYZykxUUuNFEA5HtMYhWL2JXzJqwM5tr6w2pXytL
''
---
Scheduling a call: contract_v1.identicon.testnet.register_as_validator({"validator_id":"validator07.identicon.testnet"})
Doing account.functionCall()
Receipt: HF3MC5NryCHMfBxZ1Z14BKPEwmsBkN3VHL17eCZpUDHf
        Log [contract_v1.identicon.testnet]: "validator07.identicon.testnet"
Transaction Id CnKiqQR33LDHiwJQJCHDuMRYUPjYpwwbuGWZEzHJbucb
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/CnKiqQR33LDHiwJQJCHDuMRYUPjYpwwbuGWZEzHJbucb
''
---
Scheduling a call: contract_v1.identicon.testnet.register_as_validator({"validator_id":"validator08.identicon.testnet"})
Doing account.functionCall()
Receipt: 4MtKvKuJWa9QTLw8mhMXs7ZXkyQ2RvD2HmGhbkBbejff
        Log [contract_v1.identicon.testnet]: "validator08.identicon.testnet"
Transaction Id 3uCAzYWj1pcPWVxGGtWbD1ZLdVoWeyrQn3T4w8qBgCft
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/3uCAzYWj1pcPWVxGGtWbD1ZLdVoWeyrQn3T4w8qBgCft
''
---
Scheduling a call: contract_v1.identicon.testnet.request_verification({"requestor_id":"maz.testnet", "is_type":"ProofOfLife", "subject_id":"ar_dni_12488353", "subject_info":{"age": 65, "sex":"M", "contact":{"phones":"+54-11-6549-4xxx","email": "mazito.v2@gmail.com"},"address":{"directions": "Calle Las Lomitas Nro. 23 e/ Pampa y La Via",   "city": "Adrogue","province": "Buenos Aires","country": "ar","coordinates":{"lat": "","long": ""}}}})
Doing account.functionCall()
Receipt: 4ntSGTBJnNY7SpK3hAexwPxUh8ZFXbZigUNutpDineZJ
        Log [contract_v1.identicon.testnet]: 
request_verification: Called method request_verification("maz.testnet" ProofOfLife "ar_dni_12488353")
        Log [contract_v1.identicon.testnet]: request_verification: Assign selected validators ["validator02.identicon.testnet", "validator03.identicon.testnet", "validator04.identicon.testnet"]
        Log [contract_v1.identicon.testnet]: request_verification: Assigned subject "ar_dni_12488353" to validator "validator02.identicon.testnet"
        Log [contract_v1.identicon.testnet]: request_verification: Assigned subject "ar_dni_12488353" to validator "validator03.identicon.testnet"
        Log [contract_v1.identicon.testnet]: request_verification: Assigned subject "ar_dni_12488353" to validator "validator04.identicon.testnet"
        Log [contract_v1.identicon.testnet]: request_verification: Added to verifications list VerificationRequest { is_type: ProofOfLife, requestor_id: "maz.testnet", subject_id: "ar_dni_12488353", subject_info: SubjectInfo { age: 65, sex: "M", contact: ContactInfo { phones: "+54-11-6549-4xxx", email: "mazito.v2@gmail.com" }, address: LocationInfo { directions: "Calle Las Lomitas Nro. 23 e/ Pampa y La Via", city: "Adrogue", province: "Buenos Aires", country: "ar", coordinates: GPSCoordinates { long: "", lat: "" } } }, when: TimeWindow { starts: "2022-03-28 00:00:00", ends: "2022-03-31 15:00:00" }, state: Pending, results: [VerificationResult { validator_id: "validator02.identicon.testnet", result: Pending, timestamp: "" }, VerificationResult { validator_id: "validator03.identicon.testnet", result: Pending, timestamp: "" }, VerificationResult { validator_id: "validator04.identicon.testnet", result: Pending, timestamp: "" }] }
Transaction Id DMWk6X5KvJbDdRV5cPgPv156dm4GepQtCi58RrGQWrgR
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/DMWk6X5KvJbDdRV5cPgPv156dm4GepQtCi58RrGQWrgR
''
---
Scheduling a call: contract_v1.identicon.testnet.report_verification_result({"validator_id":"validator02.identicon.testnet", "subject_id":"ar_dni_12488353", "stated":"Approved", "cause":""})
Doing account.functionCall()
Receipt: 8qtm68J6U1bc8EMpwm8QwPGwTFH8JMV6hFJhffM7Wf7a
        Log [contract_v1.identicon.testnet]: 
report_verification_result: Called method ("validator02.identicon.testnet" "ar_dni_12488353" Approved)
Transaction Id QdPj1GxUXyidQRcMPQ5umUYRJko168Shfphbw1ZquJQ
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/QdPj1GxUXyidQRcMPQ5umUYRJko168Shfphbw1ZquJQ
''
---
Scheduling a call: contract_v1.identicon.testnet.report_verification_result({"validator_id":"validator03.identicon.testnet", "subject_id":"ar_dni_12488353", "stated":"Rejected", "cause":""})
Doing account.functionCall()
Receipt: V1LfG3UPMAbJtcoLQ6MMz59u3HLVZB4pkcWKXTWZcEp
        Log [contract_v1.identicon.testnet]: 
report_verification_result: Called method ("validator03.identicon.testnet" "ar_dni_12488353" Rejected)
Transaction Id A44fjqwZSqGqwJFTZPebbHbVAzjMzaZLMw4c5PLKnZ1c
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/A44fjqwZSqGqwJFTZPebbHbVAzjMzaZLMw4c5PLKnZ1c
''
---
Scheduling a call: contract_v1.identicon.testnet.report_verification_result({"validator_id":"validator04.identicon.testnet", "subject_id":"ar_dni_12488353", "stated":"Approved", "cause":""})
Doing account.functionCall()
Retrying request to broadcast_tx_commit as it has timed out [
  'EQAAAGlkZW50aWNvbi50ZXN0bmV0AGLzmsvbpc4F7tcs+nkCYT81/wZ98yswkktWQhlhlmkxV/PpVntRAAAdAAAAY29udHJhY3RfdjEuaWRlbnRpY29uLnRlc3RuZXQ1/qnLfdcfCkwn2Nr2/4Houjr8UTdr31AV8lxP8YaCVAEAAAACGgAAAHJlcG9ydF92ZXJpZmljYXRpb25fcmVzdWx0bgAAAHsidmFsaWRhdG9yX2lkIjoidmFsaWRhdG9yMDQuaWRlbnRpY29uLnRlc3RuZXQiLCJzdWJqZWN0X2lkIjoiYXJfZG5pXzEyNDg4MzUzIiwic3RhdGVkIjoiQXBwcm92ZWQiLCJjYXVzZSI6IiJ9AOBX60gbAAAAAAAAAAAAAAAAAAAAAAAAAHBNS9n2jLbrYsnNUXccaiciDRn2LZqnHzqt3Twg8IAWHf3UK+8DZq0BDQgoArVHyAKfrK2U59fCRhC1E9vjjQA='
]
Receipt: 4k6HEWD1ZEbUXHDG9TZP6wqC3tePGQ7tqh5wBThcqYPc
        Log [contract_v1.identicon.testnet]: 
report_verification_result: Called method ("validator04.identicon.testnet" "ar_dni_12488353" Approved)
Transaction Id 8FkTRKMs9io7a2Trqu1EAugC8y1EAxq21pKP4TLUdLxc
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/8FkTRKMs9io7a2Trqu1EAugC8y1EAxq21pKP4TLUdLxc
''
---
Scheduling a call: contract_v1.identicon.testnet.pay_validators({"requestor_id":"maz.testnet", "subject_id":"ar_dni_12488353"})
Doing account.functionCall()
Receipts: 51xmPnkZtntPmFcGUpf11tsvYXhdHuJFgbvZdJ4r8x3e, 6eT6o2dxag8UWM5UX4rW35Zj3gTWxnbatMZ99LyYSgw9, D6nyWwcpPAGbXwT9TcvuaBagREAyNyonP3xERnT4BzUy, 8im6MJ72th68vzVcTuamUWm8HQwGhUdaJaQrptKckb6E
        Log [contract_v1.identicon.testnet]: 
pay_validators: Called method pay_validators("maz.testnet" "ar_dni_12488353")
        Log [contract_v1.identicon.testnet]: pay_validators: Verification found for subject_id "ar_dni_12488353" with state: Pending
        Log [contract_v1.identicon.testnet]: pay_validators: Payable validator found "validator02.identicon.testnet"
        Log [contract_v1.identicon.testnet]: pay_validators: Payable validator found "validator03.identicon.testnet"
        Log [contract_v1.identicon.testnet]: pay_validators: Payable validator found "validator04.identicon.testnet"
Transaction Id 6J259zxBBUz3EqVPpgEaXx1iaBbgjNtYNKuqtTWkmhXv
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/6J259zxBBUz3EqVPpgEaXx1iaBbgjNtYNKuqtTWkmhXv
''
---
~~~

**Paso 4**
~~~
mzito@mariodesk:~/dev/learn/near/NCAR-Bootcamp-05-2022$ git checkout ncar/maz
  Cambiado a rama 'ncar/maz'

mzito@mariodesk:~/dev/learn/near/NCAR-Bootcamp-05-2022$ ./build.sh
  Compiling identicon v0.2.0 (/home/mzito/dev/learn/near/NCAR-Bootcamp-05-2022)
  Finished release [optimized] target(s) in 3.66s
~~~

**Paso 5**
~~~
mzito@mariodesk:~/dev/learn/near/NCAR-Bootcamp-05-2022$ near deploy contract_v1.identicon.testnet --wasmFile res/identicon.wasm --initFunction "migrate" --initArgs "{}" 
This account already has a deployed contract [ 23UioJRjK79GRecyaFQXSBGhsnd7Yx4eY5WCVYv2SLQA ]. Do you want to proceed? (y/n) y
Starting deployment. Account id: contract_v1.identicon.testnet, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: res/identicon.wasm
Receipt: 8sYdiHfGNn1rchHCgSokf6yA38mURwCDEWSrQh1GpUEo
        Log [contract_v1.identicon.testnet]: 
migrate: migrating contract state to v2: [verifications], [assignments], [validators]
Transaction Id 5Etmr92YY3kMwXvHyewxBHdBbbqsyzX4J6JzDgadPMdN
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/5Etmr92YY3kMwXvHyewxBHdBbbqsyzX4J6JzDgadPMdN
Done deploying and initializing contract_v1.identicon.testnet
~~~

Verificamos en el Explorer que el metodo migrate fue llamado.
~~~
Contract deployed: contr…on.testnet
Called method: 'migrate' in contract: contr…on.testnet
Arguments: {}
Show more...

migrate: migrating contract state to v2: [verifications], [assignments], [validators]
~~~

Y realizamos una transaccion para verificar que los datos anteriores fueron efectivamente migrados y existen.
~~~
mzito@mariodesk:~/dev/learn/near/NCAR-Bootcamp-05-2022$ near call contract_v1.identicon.testnet pay_validators '{"requestor_id":"'$REQUESTOR_ID'", "subject_id":"ar_dni_12488353"}' --accountId identicon.testnet
Scheduling a call: contract_v1.identicon.testnet.pay_validators({"requestor_id":"", "subject_id":"ar_dni_12488353"})
Doing account.functionCall()
Receipts: 4VwcyLtw1hWHCsMFPYBnfeNnaKpBMnoZWwzQ1Z6bCdoi, 4i2CWBMtALTRXw6rrZLoFeC5ut1uA3Wb4KaMdJbKPNyR, FKSH2ehng7jBZsUPB8W5U1q8TpxNwiWYKUGz8SJ8ub14, ArRYanTNbWBD3gnesBayJuGoH6vyTu9xm5iAWndcExfZ
        Log [contract_v1.identicon.testnet]: 
pay_validators: Called method pay_validators("" "ar_dni_12488353")
        Log [contract_v1.identicon.testnet]: pay_validators: Verification found for subject_id "ar_dni_12488353" with state: Pending
        Log [contract_v1.identicon.testnet]: pay_validators: Payable validator found "validator02.identicon.testnet"
        Log [contract_v1.identicon.testnet]: pay_validators: Payable validator found "validator03.identicon.testnet"
        Log [contract_v1.identicon.testnet]: pay_validators: Payable validator found "validator04.identicon.testnet"
Transaction Id 2tdSuqfEetgSM6JgDsvWa5XTP2utXjYg6aNjV8f8otW8
To see the transaction in the transaction explorer, please open this url in your browser https://explorer.testnet.near.org/transactions/2tdSuqfEetgSM6JgDsvWa5XTP2utXjYg6aNjV8f8otW8
~~~

### Actualizar contrato desde SputnikDAO

Instalar `CLI SputnikDAO v2` en otra carpeta y generar link:
~~~
mkdir ~/dev/learn/near/identicondao
cd ~/dev/learn/near/identicondao
git clone https://github.com/cloudmex/sputnikdao-cli.git ./
rm -rf node_modules
rm package-lock.json 
npm i
npm run build
npm link
~~~

Volver a este espacio y vincular el CLI:
~~~
cd ~/dev/learn/near/NCAR-Bootcamp-05-2022
sudo npm link sputnikdao
~~~

Ya podemos usar `CLI sputnikdao`:
~~~
mzito@mariodesk:~/dev/learn/near/NCAR-Bootcamp-05-2022$ sputnikdao proposal upgrade res/identicon.wasm contract_v1.identicon.testnet --daoAcc identicon --accountId identicon.testnet
call identicon.sputnikv2.testnet store_blob [Uint8Array] --accountId identicon.testnet -gas:200 --amount:2.204

result.status.SuccessValue: "8JJKuKYBSJsgvjaU2MfyDLGdkH8wAhfV6dkdiPeofGWQ"
call identicon.sputnikv2.testnet add_proposal {"proposal":{"target":"contract_v1.identicon.testnet","description":"Upgrade code","kind":{"UpgradeRemote":{"receiver_id":"contract_v1.identicon.testnet","method_name":"upgrade","hash":"8JJKuKYBSJsgvjaU2MfyDLGdkH8wAhfV6dkdiPeofGWQ"}}}} --accountId identicon.testnet -gas:200 --amount:1

result.status.SuccessValue: 0
0
~~~

Aprobar la propuesta:

![Captura de propuesta aprobada](https://github.com/identicondapp/identicon/blob/ncar/maz/docs/maz/Captura-2022-05-11.png)

Verificar transacciones en Explorer:
~~~
Called method: 'act_proposal' in contract: ident…v2.testnet
by maz.testnet
H8GJZDn... Succeeded 1hr ago

Called method: 'add_proposal' in contract: ident…v2.testnet
by identicon.testnet
5M1V5rr... Succeeded 1hr ago

Called method: 'store_blob' in contract: ident…v2.testnet 
by identicon.testnet
ExjjZZj... Succeeded 1hr ago

New key added for identicon.testnet: ed25519:EfdQ8F4... with permission FullAccess
by identicon.testnet
76mm9FD... Succeeded 1hr ago

New key added for identicon.testnet: ed25519:AMYMBf2... with permission FullAccess
by identicon.testnet
5da5cjb... Succeeded 2hr ago

Called method: 'create' in contract: sputnikv2.testnet 
by identicon.testnet
72Tkifh... Succeeded 2hr ago

Access key added for contract sputnikv2.testnet: ed25519:H4ysynM... with permission to call any methods
by identicon.testnet
5HAsCAG... Succeeded 2hr ago
~~~
