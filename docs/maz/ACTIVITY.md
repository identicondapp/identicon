# NCAR - Bootcamp 09-05-2022 al 13-05-2022

### Día 1 (Martes 10-May)

**A realizar**

- `HECHO` Clona el repositorio de Github, compila y despliega el contrato. 
- Crea la estructura de archivos para tu contrato inteligente, es decir, los archivos migrate.rs, internals.rs, enumerations.rs y los que consideres necesarios.
- `PROCESO` Implementa las buenas prácticas recomendadas por el Protocolo de NEAR para el lenguaje de programación Rust. 
- `HECHO` Corrige el archivo **Cargo.toml** para optimizar el peso del archivo compilado.
- ¡Compila y Despliega tu contrato para realizar las pruebas necesarias y seguir añadiendo las herramientas para escalabilidad y mantenimiento para tu DApp!

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
  Downloaded num-integer v0.1.45
  Downloaded num-traits v0.2.15
  Downloaded semver v1.0.9
  Downloaded serde_derive v1.0.137
  Downloaded thiserror-impl v1.0.31
  Downloaded syn v1.0.93
  Downloaded unicode-xid v0.2.3
  Downloaded thiserror v1.0.31
  Downloaded tinyvec v1.6.0
  Downloaded serde v1.0.137
  Downloaded ed25519 v1.5.0
  Downloaded unicode-bidi v0.3.8
  Downloaded memchr v2.5.0
  Downloaded quote v1.0.18
  Downloaded serde_json v1.0.81
  Downloaded toml v0.5.9
  Downloaded proc-macro2 v1.0.38
  Downloaded libc v0.2.125
  Downloaded near-sdk-macros v4.0.0-pre.8
  Downloaded near-sdk v4.0.0-pre.8
  Downloaded 20 crates (1.7 MB) in 1.99s
   Compiling proc-macro2 v1.0.38
   Compiling unicode-xid v0.2.3
   Compiling syn v1.0.93
   Compiling libc v0.2.125
   Compiling version_check v0.9.4
   Compiling cfg-if v1.0.0
   Compiling serde_derive v1.0.137
   Compiling serde v1.0.137
   Compiling autocfg v1.1.0
   Compiling typenum v1.15.0
   Compiling once_cell v1.10.0
   Compiling ppv-lite86 v0.2.16
   Compiling getrandom v0.1.16
   Compiling radium v0.6.2
   Compiling opaque-debug v0.3.0
   Compiling serde_json v1.0.81
   Compiling block-padding v0.2.1
   Compiling byteorder v1.4.3
   Compiling itoa v1.0.1
   Compiling ryu v1.0.9
   Compiling funty v1.1.0
   Compiling cfg-if v0.1.10
   Compiling tinyvec_macros v0.1.0
   Compiling crunchy v0.2.2
   Compiling hex v0.4.3
   Compiling memchr v2.5.0
   Compiling wyz v0.2.0
   Compiling subtle v2.4.1
   Compiling tap v1.0.1
   Compiling arrayvec v0.7.2
   Compiling static_assertions v1.1.0
   Compiling cpufeatures v0.2.2
   Compiling cc v1.0.73
   Compiling matches v0.1.9
   Compiling byte-slice-cast v1.2.1
   Compiling percent-encoding v2.1.0
   Compiling unicode-bidi v0.3.8
   Compiling rustc-hex v2.1.0
   Compiling signature v1.5.0
   Compiling convert_case v0.4.0
   Compiling lazy_static v1.4.0
   Compiling bs58 v0.4.0
   Compiling regex-syntax v0.6.25
   Compiling reed-solomon-erasure v4.0.2
   Compiling arrayvec v0.5.2
   Compiling base64 v0.11.0
   Compiling smallvec v1.8.0
   Compiling arrayref v0.3.6
   Compiling validator_types v0.12.0
   Compiling bytesize v1.1.0
   Compiling base64 v0.13.0
   Compiling wee_alloc v0.4.5
   Compiling easy-ext v0.2.9
   Compiling keccak v0.1.0
   Compiling Inflector v0.11.4
   Compiling memory_units v0.4.0
   Compiling near-sys v0.1.0
   Compiling generic-array v0.14.5
   Compiling ahash v0.7.6
   Compiling indexmap v1.8.1
   Compiling num-traits v0.2.15
   Compiling num-integer v0.1.45
   Compiling num-bigint v0.3.3
   Compiling num-rational v0.3.2
   Compiling tinyvec v1.6.0
   Compiling form_urlencoded v1.0.1
   Compiling ed25519 v1.5.0
   Compiling parity-secp256k1 v0.7.0
   Compiling quote v1.0.18
   Compiling unicode-normalization v0.1.19
   Compiling getrandom v0.2.6
   Compiling time v0.1.43
   Compiling bitvec v0.20.4
   Compiling uint v0.9.3
   Compiling aho-corasick v0.7.18
   Compiling rand_core v0.6.3
   Compiling rand_core v0.5.1
   Compiling idna v0.2.3
   Compiling digest v0.9.0
   Compiling block-buffer v0.9.0
   Compiling cipher v0.2.5
   Compiling crypto-mac v0.8.0
   Compiling regex v1.5.5
   Compiling hashbrown v0.11.2
   Compiling rand_chacha v0.3.1
   Compiling rand_chacha v0.2.2
   Compiling sha2 v0.9.9
   Compiling ripemd160 v0.9.1
   Compiling sha3 v0.9.1
   Compiling url v2.2.2
   Compiling c2-chacha v0.3.3
   Compiling blake2 v0.9.2
   Compiling rand v0.8.5
   Compiling rand v0.7.3
   Compiling borsh-schema-derive-internal v0.9.3
   Compiling borsh-derive-internal v0.9.3
   Compiling synstructure v0.12.6
   Compiling fixed-hash v0.7.0
   Compiling thiserror-impl v1.0.31
   Compiling zeroize_derive v1.3.2
   Compiling impl-trait-for-tuples v0.2.2
   Compiling derive_more v0.99.17
   Compiling smart-default v0.6.0
   Compiling near-sdk-macros v4.0.0-pre.8
   Compiling zeroize v1.3.0
   Compiling thiserror v1.0.31
   Compiling curve25519-dalek v3.2.1
   Compiling toml v0.5.9
   Compiling near-rpc-error-core v0.10.0
   Compiling ed25519-dalek v1.0.1
   Compiling chrono v0.4.19
   Compiling validator v0.12.0
   Compiling proc-macro-crate v0.1.5
   Compiling proc-macro-crate v1.1.3
   Compiling near-rpc-error-macro v0.10.0
   Compiling borsh-derive v0.9.3
   Compiling parity-scale-codec-derive v2.3.1
   Compiling borsh v0.9.3
   Compiling near-account-id v0.10.0
   Compiling parity-scale-codec v2.3.1
   Compiling near-vm-errors v0.10.0
   Compiling near-primitives-core v0.10.0
   Compiling impl-codec v0.5.1
   Compiling primitive-types v0.10.1
   Compiling near-crypto v0.10.0
   Compiling near-primitives v0.10.0
   Compiling near-vm-logic v0.10.0
   Compiling near-sdk v4.0.0-pre.8
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

**Implementar buenas pácticas en código**

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


