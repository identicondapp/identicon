
# Actividades Día 4 (Viernes 13-May)

### Implementar XCC sobre `request_verification` en `contract_v2` 

Se hace el llamado usando un metodo de test:
~~~rust
    #[test]
    fn test_xcc_request_verification_low_level() {
        // Basic set up for the unit test
        testing_env!(VMContextBuilder::new().build());
        let subject_id = "ar_dni_12488401".to_string();
        let requestor_id = "maz.testnet".to_string();
        let subject_info = &moq_subject_info();
        let contract_name = "contract_v2.identicon.testnet";
        let method_name = "request_verification";
        
        // We will call this method  but on an different version of the already 
        // deployed contract (contract_v2)
        // contract.request_verification(
        //     requestor_id.to_string(),
        //     VerificationType::ProofOfLife,
        //     subject_id.to_string(),
        //     moq_subject_info(),
        // );
        // Prepaid gas for a single  call.
        const XCC_CALL_GAS: Gas = Gas(20_000_000_000_000);
        let prepaid_gas = env::prepaid_gas() - XCC_CALL_GAS;

        // set the AccountId to be used in the XCC call
        // which for our case is the deployed contract 
        let contract_id: AccountId = contract_name.parse().unwrap();

        // but we need to serialize the params
        let params: Vec<u8> = serde_json::to_vec(&(  
          requestor_id.to_string(),
          VerificationType::ProofOfLife,
          subject_id.to_string(),
          subject_info,
        )).unwrap();

        // make the XCC call 
        let promise_idx = env::promise_create(
          contract_id.clone(),  // the account id
          method_name,          // the method to call on the contract
          &params,              // the serialized method params
          0,
          prepaid_gas,
        );        
        
        log!("\ntest_xcc_request_verification_result_low_level: promise_id={:?} \ncall={:?} method{:?} \nserialized={:?}", 
          promise_idx,
          contract_id, 
          method_name, 
          params.clone(), 
        );
    }
~~~

Aqui se corre el test:
~~~shell
mzito@mariodesk:~/dev/learn/near/NCAR-Bootcamp-05-2022$ ./test.sh test_xcc_request_verification_low_level
   Compiling identicon v0.2.0 (/home/mzito/dev/learn/near/NCAR-Bootcamp-05-2022)

warning: `identicon` (lib test) generated 2 warnings
    Finished test [unoptimized + debuginfo] target(s) in 1.66s
     Running unittests (target/debug/deps/identicon-8b52b262c817c864)

running 1 test

test_xcc_request_verification_result: 
call=AccountId("contract_v2.identicon.testnet") method"request_verification" 
serialized=[91, 34, 109, 97, 122, 46, 116, 101, 115, 116, 110, 101, 116, 34, 44, 34, 80, 114, 111, 111, 102, 79, 102, 76, 105, 102, 101, 34, 44, 34, 97, 114, 95, 100, 110, 105, 95, 49, 50, 52, 56, 56, 52, 48, 49, 34, 44, 123, 34, 97, 103, 101, 34, 58, 54, 53, 44, 34, 115, 101, 120, 34, 58, 34, 77, 34, 44, 34, 99, 111, 110, 116, 97, 99, 116, 34, 58, 123, 34, 112, 104, 111, 110, 101, 115, 34, 58, 34, 43, 53, 52, 45, 49, 49, 45, 54, 53, 52, 57, 45, 52, 120, 120, 120, 34, 44, 34, 101, 109, 97, 105, 108, 34, 58, 34, 109, 97, 122, 105, 116, 111, 46, 118, 50, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 34, 125, 44, 34, 97, 100, 100, 114, 101, 115, 115, 34, 58, 123, 34, 100, 105, 114, 101, 99, 116, 105, 111, 110, 115, 34, 58, 34, 67, 97, 108, 108, 101, 32, 76, 97, 115, 32, 76, 111, 109, 105, 116, 97, 115, 32, 78, 114, 111, 46, 32, 50, 51, 32, 101, 47, 32, 80, 97, 109, 112, 97, 32, 121, 32, 76, 97, 32, 86, 105, 97, 34, 44, 34, 99, 105, 116, 121, 34, 58, 34, 65, 100, 114, 111, 103, 117, 101, 34, 44, 34, 112, 114, 111, 118, 105, 110, 99, 101, 34, 58, 34, 66, 117, 101, 110, 111, 115, 32, 65, 105, 114, 101, 115, 34, 44, 34, 99, 111, 117, 110, 116, 114, 121, 34, 58, 34, 97, 114, 34, 44, 34, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 34, 58, 123, 34, 108, 111, 110, 103, 34, 58, 34, 34, 44, 34, 108, 97, 116, 34, 58, 34, 34, 125, 125, 125, 93]
test tests::tests::test_xcc_request_verification_low_level ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s
~~~

**Falta:** Todavía no se procesa el `callback` del llamado y no se hace tratamiento de errores.


### Agregar un IPFS 

Abrimos una cuenta en Pinata: https://app.pinata.cloud/pinmanager

Subimos un archivo, cuyo CID es:
~~~
Qmc3kQzgwWof7mLG7PPPb1vx6DYDqv9YrW1nWBGhHRqiWW
~~~

Se puede acceder mediante el [gateway IPFS de Pinata](https://gateway.pinata.cloud/ipfs) usando su CID:
~~~
https://gateway.pinata.cloud/ipfs/Qmc3kQzgwWof7mLG7PPPb1vx6DYDqv9YrW1nWBGhHRqiWW
~~~

Agregamos el metodo `bind_card_file` al contrato para vinculara el archivo al sujeto verificado,
agregamos un test unitario para el metodo y corremos el test:
~~~
$ ./test.sh bind_card_file

warning: `identicon` (lib test) generated 4 warnings
    Finished test [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests (target/debug/deps/identicon-8b52b262c817c864)

running 1 test

init:: initialized contract state: [verifications], [assignments], [validators]

bind_card_file: Called method bind_card_file("ar_dni_12488401" "Qmc3kQzgwWof7mLG7PPPb1vx6DYDqv9YrW1nWBGhHRqiWW")

test_bind_card_file: subject_id="ar_dni_12488401" card_id="Qmc3kQzgwWof7mLG7PPPb1vx6DYDqv9YrW1nWBGhHRqiWW"
test tests::tests::test_bind_card_file ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s
~~~

Ahora llamamos al metodo de nuestro contrato usando near cli:
~~~
$ ./run-bindcard.sh identicon.testnet 488
Scheduling a call: contract_v3.identicon.testnet.bind_card_file({"subject_id":"ar_dni_12488488", "card_id":"Qmc3kQzgwWof7mLG7PPPb1vx6DYDqv9YrW1nWBGhHRqiWW"})
Doing account.functionCall()
Receipt: CiUp7ZpQwENHt9nTa5QbzRY8bq2YuRZvYHYMiwrRMac5
        Log [contract_v3.identicon.testnet]: 
bind_card_file: Called method bind_card_file("ar_dni_12488488" "Qmc3kQzgwWof7mLG7PPPb1vx6DYDqv9YrW1nWBGhHRqiWW")
Transaction Id 2dyACfBzaDh5EfDefsHe3C7P7syjD5Uqic4z9Wzh5iy1
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/2dyACfBzaDh5EfDefsHe3C7P7syjD5Uqic4z9Wzh5iy1
''
~~~
