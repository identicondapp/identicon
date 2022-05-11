
# Actividades Día 1 (Martes 10-May)

**Inicial**

Clonar repo inicial para comenzar proyecto:
~~~
git clone git@github.com:identicondapp/identicon.git ~/dev/learn/NCAR-Bootcamp-05-2022
~~~

Crear nuevo branch para separar actividades de cada participante:
~~~
git checkout main
git branch ncar/maz
git checkout ncar/maz
git push --force origin ncar/maz
~~~

Agregar carpetas de documentación para cada participante y commit

**Cambios en `Cargo.toml`**

Verificar que estan aplicados los cambios en `Cargo.toml`

- `crate-type` no contiene `rlib`: OK
- `opt-level=z`: OK
- `lto = true`: OK
- `debug = false`: OK
- `panic = "abort"`: OK
- `overflow-checks = true`: OK

Se modifica `near-sdk = "=4.0.0-pre.5"` basado en ejemplo provisto.

Se agrega `serde_json = "1.0"`y `near-sys = "0.1.0"`

Se compiló y corrieron todos los test unitarios: OK
~~~
mzito@mariodesk:~/dev/learn/near/NCAR-Bootcamp-05-2022$ ./test.sh
    Updating crates.io index
   Compiling identicon v0.2.0 (/home/mzito/dev/learn/near/NCAR-Bootcamp-05-2022)
    Finished test [unoptimized + debuginfo] target(s) in 4m 04s
     Running unittests (target/debug/deps/identicon-8b52b262c817c864)

running 4 tests
"validator01.testnet"
"validator02.testnet"
"validator03.testnet"
"validator04.testnet"
"validator05.testnet"

---
test_register_validator 5 ["validator01.testnet", "validator02.testnet", "validator03.testnet", "validator04.testnet", "validator05.testnet"]
test tests::test_register_validators ... ok

pay_validators: Called method pay_validators("requestor01.testnet" "subject01.testnet")
"validator01.testnet"
"validator02.testnet"
"validator03.testnet"
"validator04.testnet"
"validator05.testnet"
"validator06.testnet"
"validator07.testnet"
"validator08.testnet"
"validator09.testnet"
"validator10.testnet"
"validator11.testnet"

request_verification: Called method request_verification("requestor01.testnet" ProofOfLife "ar_dni_12488353")
request_verification: Assign selected validators ["validator02.testnet", "validator03.testnet", "validator04.testnet"]
request_verification: Assigned subject "ar_dni_12488353" to validator "validator02.testnet"
request_verification: Assigned subject "ar_dni_12488353" to validator "validator03.testnet"
request_verification: Assigned subject "ar_dni_12488353" to validator "validator04.testnet"
request_verification: Added to verifications list VerificationRequest { is_type: ProofOfLife, requestor_id: "requestor01.testnet", subject_id: "ar_dni_12488353", subject_info: SubjectInfo { age: 65, sex: "M", contact: ContactInfo { phones: "+54-11-6549-4xxx", email: "mazito.v2@gmail.com" }, address: LocationInfo { directions: "Calle Las Lomitas Nro. 23 e/ Pampa y La Via", city: "Adrogue", province: "Buenos Aires", country: "ar", coordinates: GPSCoordinates { long: "", lat: "" } } }, when: TimeWindow { starts: "2022-03-28 00:00:00", ends: "2022-03-31 15:00:00" }, state: Pending, results: [VerificationResult { validator_id: "validator02.testnet", result: Pending, timestamp: "" }, VerificationResult { validator_id: "validator03.testnet", result: Pending, timestamp: "" }, VerificationResult { validator_id: "validator04.testnet", result: Pending, timestamp: "" }] }
pay_validators: Verification found for subject_id "subject01.testnet" with state: Pending
"validator01.testnet"
"validator02.testnet"
"validator03.testnet"
"validator04.testnet"
"validator05.testnet"
"validator06.testnet"
"validator07.testnet"
"validator08.testnet"
"validator09.testnet"
"validator10.testnet"
"validator11.testnet"

request_verification: Called method request_verification("requestor01.testnet" ProofOfLife "ar_dni_12488353")
request_verification: Assign selected validators ["validator02.testnet", "validator03.testnet", "validator04.testnet"]
request_verification: Assigned subject "ar_dni_12488353" to validator "validator02.testnet"
request_verification: Assigned subject "ar_dni_12488353" to validator "validator03.testnet"
request_verification: Assigned subject "ar_dni_12488353" to validator "validator04.testnet"
request_verification: Added to verifications list VerificationRequest { is_type: ProofOfLife, requestor_id: "requestor01.testnet", subject_id: "ar_dni_12488353", subject_info: SubjectInfo { age: 65, sex: "M", contact: ContactInfo { phones: "+54-11-6549-4xxx", email: "mazito.v2@gmail.com" }, address: LocationInfo { directions: "Calle Las Lomitas Nro. 23 e/ Pampa y La Via", city: "Adrogue", province: "Buenos Aires", country: "ar", coordinates: GPSCoordinates { long: "", lat: "" } } }, when: TimeWindow { starts: "2022-03-28 00:00:00", ends: "2022-03-31 15:00:00" }, state: Pending, results: [VerificationResult { validator_id: "validator02.testnet", result: Pending, timestamp: "" }, VerificationResult { validator_id: "validator03.testnet", result: Pending, timestamp: "" }, VerificationResult { validator_id: "validator04.testnet", result: Pending, timestamp: "" }] }

report_verification_result: Called method ("validator03.testnet" "ar_dni_12488353" Approved)
pay_validators: Payable validator found "validator01.testnet"

report_verification_result: Called method ("validator04.testnet" "ar_dni_12488353" Approved)
pay_validators: Payable validator found "validator02.testnet"

report_verification_result: Called method ("validator02.testnet" "ar_dni_12488353" Approved)

test_report_verification_result: [VerificationResult { validator_id: "validator02.testnet", result: Approved, timestamp: "2022-03-31 16:00:00" }, VerificationResult { validator_id: "validator03.testnet", result: Approved, timestamp: "2022-03-31 16:00:00" }, VerificationResult { validator_id: "validator04.testnet", result: Approved, timestamp: "2022-03-31 16:00:00" }]
test tests::test_request_verification ... ok
test tests::test_report_verification_result ... ok
test tests::test_pay_validators ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
~~~

**Implementar buenas prácticas en código**

FLAGS: `overflow-checks=true`: OK (ya estaba en Cargo.toml)

Problema al refactorizar codigo:
~~~
// this DOES NOT WORK, why ?
use definitions::{
  MAX_VALIDATORS, MIN_VALIDATORS, PRIZE_AMOUNT,
  SubjectId, RequestorId, ValidatorId, ISODateTime,
  GPSCoordinates, ContactInfo, LocationInfo, TimeWindow, SubjectInfo,
  VerificationType, VerificationState, VerificationResult, VerificationRequest
  , VerificationContract
};

// but this works, why ?
use definitions::*; 
~~~

Se refactorizó el código en:

- `lib.rs`: root del proyecto que implementa inicializacion del contrato.
- `definitions.rs`: definiciones de datos (tipos, structs y constantes).
- `requests.rs`: metodos usados por el **solicitante**.
- `validators.rs`: metodos usado por los **validadores**.
- `payments.rs`: contiene el pago a validadores, incluyendo Promise y XCC.
- `test.rs`: tests unitarios.

Se verifico el uso de `assert!` y `log!` en la mayoría de los metodos implementados.

Se corrieron los test unitarios en forma continua durante la refactorización y al final:
~~~
mzito@mariodesk:~/dev/learn/near/NCAR-Bootcamp-05-2022$ cargo fmt
mzito@mariodesk:~/dev/learn/near/NCAR-Bootcamp-05-2022$ ./test.sh
   Compiling identicon v0.2.0 (/home/mzito/dev/learn/near/NCAR-Bootcamp-05-2022)
    Finished test [unoptimized + debuginfo] target(s) in 1.61s
     Running unittests (target/debug/deps/identicon-8b52b262c817c864)

running 4 tests

initialized contract state: [verifications], [assignments], [validators]

pay_validators: Called method pay_validators("requestor01.testnet" "subject01.testnet")
pay_validators: Verification found for subject_id "subject01.testnet" with state: Pending
pay_validators: Payable validator found "validator01.testnet"

initialized contract state: [verifications], [assignments], [validators]
"validator01.testnet"
"validator02.testnet"
"validator03.testnet"
"validator04.testnet"
"validator05.testnet"
"validator06.testnet"
"validator07.testnet"
"validator08.testnet"
"validator09.testnet"
"validator10.testnet"
"validator11.testnet"

request_verification: Called method request_verification("requestor01.testnet" ProofOfLife "ar_dni_12488353")
request_verification: Assign selected validators ["validator02.testnet", "validator03.testnet", "validator04.testnet"]
request_verification: Assigned subject "ar_dni_12488353" to validator "validator02.testnet"

initialized contract state: [verifications], [assignments], [validators]
"validator01.testnet"
"validator02.testnet"
"validator03.testnet"
"validator04.testnet"
request_verification: Assigned subject "ar_dni_12488353" to validator "validator03.testnet"
request_verification: Assigned subject "ar_dni_12488353" to validator "validator04.testnet"
request_verification: Added to verifications list VerificationRequest { is_type: ProofOfLife, requestor_id: "requestor01.testnet", subject_id: "ar_dni_12488353", subject_info: SubjectInfo { age: 65, sex: "M", contact: ContactInfo { phones: "+54-11-6549-4xxx", email: "mazito.v2@gmail.com" }, address: LocationInfo { directions: "Calle Las Lomitas Nro. 23 e/ Pampa y La Via", city: "Adrogue", province: "Buenos Aires", country: "ar", coordinates: GPSCoordinates { long: "", lat: "" } } }, when: TimeWindow { starts: "2022-03-28 00:00:00", ends: "2022-03-31 15:00:00" }, state: Pending, results: [VerificationResult { validator_id: "validator02.testnet", result: Pending, timestamp: "" }, VerificationResult { validator_id: "validator03.testnet", result: Pending, timestamp: "" }, VerificationResult { validator_id: "validator04.testnet", result: Pending, timestamp: "" }] }
test tests::tests::test_request_verification ... "validator05.testnet"
ok

---
test_register_validator 5 ["validator01.testnet", "validator02.testnet", "validator03.testnet", "validator04.testnet", "validator05.testnet"]

initialized contract state: [verifications], [assignments], [validators]
test tests::tests::test_register_validators ... ok"validator01.testnet"

"validator02.testnet"
"validator03.testnet"
"validator04.testnet"
"validator05.testnet"
"validator06.testnet"
"validator07.testnet"
"validator08.testnet"
"validator09.testnet"
"validator10.testnet"
"validator11.testnet"

request_verification: Called method request_verification("requestor01.testnet" ProofOfLife "ar_dni_12488353")
request_verification: Assign selected validators ["validator02.testnet", "validator03.testnet", "validator04.testnet"]
request_verification: Assigned subject "ar_dni_12488353" to validator "validator02.testnet"
request_verification: Assigned subject "ar_dni_12488353" to validator "validator03.testnet"
request_verification: Assigned subject "ar_dni_12488353" to validator "validator04.testnet"
request_verification: Added to verifications list VerificationRequest { is_type: ProofOfLife, requestor_id: "requestor01.testnet", subject_id: "ar_dni_12488353", subject_info: SubjectInfo { age: 65, sex: "M", contact: ContactInfo { phones: "+54-11-6549-4xxx", email: "mazito.v2@gmail.com" }, address: LocationInfo { directions: "Calle Las Lomitas Nro. 23 e/ Pampa y La Via", city: "Adrogue", province: "Buenos Aires", country: "ar", coordinates: GPSCoordinates { long: "", lat: "" } } }, when: TimeWindow { starts: "2022-03-28 00:00:00", ends: "2022-03-31 15:00:00" }, state: Pending, results: [VerificationResult { validator_id: "validator02.testnet", result: Pending, timestamp: "" }, VerificationResult { validator_id: "validator03.testnet", result: Pending, timestamp: "" }, VerificationResult { validator_id: "validator04.testnet", result: Pending, timestamp: "" }] }

report_verification_result: Called method ("validator03.testnet" "ar_dni_12488353" Approved)
pay_validators: Payable validator found "validator02.testnet"

report_verification_result: Called method ("validator04.testnet" "ar_dni_12488353" Approved)
test tests::tests::test_pay_validators ... ok

report_verification_result: Called method ("validator02.testnet" "ar_dni_12488353" Approved)

test_report_verification_result: [VerificationResult { validator_id: "validator02.testnet", result: Approved, timestamp: "2022-03-31 16:00:00" }, VerificationResult { validator_id: "validator03.testnet", result: Approved, timestamp: "2022-03-31 16:00:00" }, VerificationResult { validator_id: "validator04.testnet", result: Approved, timestamp: "2022-03-31 16:00:00" }]
test tests::tests::test_report_verification_result ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s 
~~~

Deploy del contrato `contract0` sobre cuenta `identicon.testnet`:
~~~
mzito@mariodesk:~/dev/learn/near/NCAR-Bootcamp-05-2022$ ./deploy.sh identicon.testnet contract0.identicon.testnet
    Finished release [optimized] target(s) in 0.04s
Account contract0.identicon.testnet for network "testnet" was created.
Starting deployment. Account id: contract0.identicon.testnet, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: res/identicon.wasm
Transaction Id HxFAcmWWpHgha7q9oyHfRByLthYS2GykKZteHZUXbaRL
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/HxFAcmWWpHgha7q9oyHfRByLthYS2GykKZteHZUXbaRL
Done deploying to contract0.identicon.testnet
~~~
