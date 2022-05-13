use crate::definitions::*;
use near_sdk::serde_json;
use near_sdk::{env, log, Gas, Promise, PromiseResult};

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package identicon -- --nocapture
 * Note: 'identicon' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    #[allow(dead_code)]
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[allow(dead_code)]
    fn moq_request_data() -> VerificationRequest {
        let request = VerificationRequest {
            is_type: VerificationType::ProofOfLife,
            requestor_id: "identicon.testnet".to_string(),
            subject_id: "subject01".to_string(),
            subject_info: moq_subject_info(),
            when: moq_time_window(),
            state: VerificationState::Pending,
            results: vec![
                VerificationResult {
                    validator_id: "validator01.testnet".to_string(),
                    result: VerificationState::Approved,
                    timestamp: "".to_string(),
                },
                VerificationResult {
                    validator_id: "validator02.testnet".to_string(),
                    result: VerificationState::Rejected,
                    timestamp: "".to_string(),
                },
                VerificationResult {
                    validator_id: "validator03.testnet".to_string(),
                    result: VerificationState::Pending,
                    timestamp: "".to_string(),
                },
            ],
        };
        request
    }

    #[allow(dead_code)]
    fn moq_subject_info() -> SubjectInfo {
        let subject_info = SubjectInfo {
            age: 65,
            sex: "M".to_string(),
            contact: ContactInfo {
                phones: "+54-11-6549-4xxx".to_string(),
                email: "mazito.v2@gmail.com".to_string(),
            },
            address: LocationInfo {
                directions: "Calle Las Lomitas Nro. 23 e/ Pampa y La Via".to_string(),
                city: "Adrogue".to_string(),
                province: "Buenos Aires".to_string(),
                country: "ar".to_string(),
                coordinates: GPSCoordinates {
                    lat: "".to_string(),
                    long: "".to_string(),
                },
            },
        };
        subject_info
    }

    #[allow(dead_code)]
    fn moq_time_window() -> TimeWindow {
        let when = TimeWindow {
            starts: "2022-03-28 00:00:00".to_string(),
            ends: "2022-03-31 15:00:00".to_string(),
        };
        when
    }

    #[allow(dead_code)]
    fn moq_request_data_with_params(
        requestor_id: RequestorId,
        subject_id: SubjectId,
        validators: Vec<ValidatorId>,
    ) -> VerificationRequest {
        let mut request = VerificationRequest {
            is_type: VerificationType::ProofOfLife,
            requestor_id: requestor_id.to_string(),
            subject_id: subject_id.to_string(),
            subject_info: moq_subject_info(),
            when: moq_time_window(),
            state: VerificationState::Pending,
            results: Vec::new(),
        };

        for validator in validators.iter() {
            request.results.push(VerificationResult {
                validator_id: validator.to_string(),
                result: VerificationState::Pending,
                timestamp: "".to_string(),
            });
        }

        request
    }

    #[allow(dead_code)]
    fn moq_contract_data(mut contract: VerificationContract) -> VerificationContract {
        let request = moq_request_data();
        contract
            .verifications
            .insert(&"subject01.testnet".to_string(), &request);

        contract.assignments.insert(
            &"validator01.testnet".to_string(),
            &vec!["subject01.testnet".to_string()],
        );

        contract.validators = vec![
            "validator01.testnet".to_string(),
            "validator02.testnet".to_string(),
            "validator03.testnet".to_string(),
            "validator04.testnet".to_string(),
            "validator05.testnet".to_string(),
            "validator06.testnet".to_string(),
        ];

        contract
    }

    #[allow(dead_code)]
    fn moq_validators_pool(mut contract: VerificationContract) -> VerificationContract {
        contract.register_as_validator("validator01.testnet".to_string());
        contract.register_as_validator("validator02.testnet".to_string());
        contract.register_as_validator("validator03.testnet".to_string());
        contract.register_as_validator("validator04.testnet".to_string());
        contract.register_as_validator("validator05.testnet".to_string());
        contract.register_as_validator("validator06.testnet".to_string());
        contract.register_as_validator("validator07.testnet".to_string());
        contract.register_as_validator("validator08.testnet".to_string());
        contract.register_as_validator("validator09.testnet".to_string());
        contract.register_as_validator("validator10.testnet".to_string());
        contract.register_as_validator("validator11.testnet".to_string());
        contract
    }

    #[test]
    fn test_request_verification() {
        // Basic set up for a unit test
        testing_env!(VMContextBuilder::new().build());
        let subject_id = "ar_dni_12488353".to_string();
        let requestor_id = "requestor01.testnet".to_string();
        let mut contract = VerificationContract::new();

        contract = moq_validators_pool(contract);

        contract.request_verification(
            requestor_id.to_string(),
            VerificationType::ProofOfLife,
            subject_id.to_string(),
            moq_subject_info(),
        );

        let request = contract.verifications.get(&subject_id.to_string()).unwrap();
        assert_eq!(request.requestor_id, requestor_id);
        assert_eq!(request.subject_id, subject_id);
        assert_eq!(request.results.len(), 3);
        assert_eq!(contract.verifications.len(), 1);
        assert_eq!(contract.assignments.len(), 3);
    }

    #[test]
    fn test_pay_validators() {
        // Basic set up for a unit test
        testing_env!(VMContextBuilder::new().build());
        let contract = VerificationContract::new();
        let mut contract1 = moq_contract_data(contract);

        contract1.pay_validators(
            "requestor01.testnet".to_string(),
            "subject01.testnet".to_string(),
        );
    }

    #[test]
    fn test_register_validators() {
        // Basic set up for a unit test
        testing_env!(VMContextBuilder::new().build());
        let mut contract = VerificationContract::new();

        contract.register_as_validator("validator01.testnet".to_string());
        contract.register_as_validator("validator02.testnet".to_string());
        contract.register_as_validator("validator03.testnet".to_string());
        contract.register_as_validator("validator04.testnet".to_string());
        contract.register_as_validator("validator05.testnet".to_string());

        log!(
            "\n---\ntest_register_validator {:?} {:?}",
            contract.get_validators_count(),
            contract.validators
        );
        assert_eq!(5, contract.get_validators_count());
    }

    #[test]
    fn test_report_verification_result() {
        // Basic set up for a unit test
        testing_env!(VMContextBuilder::new().build());
        let subject_id = "ar_dni_12488353".to_string();
        let requestor_id = "requestor01.testnet".to_string();
        let valid0 = "validator02.testnet".to_string();
        let valid1 = "validator03.testnet".to_string();
        let valid2 = "validator04.testnet".to_string();
        let mut contract = VerificationContract::new();

        contract = moq_validators_pool(contract);

        contract.request_verification(
            requestor_id.to_string(),
            VerificationType::ProofOfLife,
            subject_id.to_string(),
            moq_subject_info(),
        );

        // the second validator approved it
        contract.report_verification_result(
            valid1.to_string(),
            subject_id.to_string(),
            VerificationState::Approved,
            "".to_string(),
        );
        let request = contract.verifications.get(&subject_id).unwrap();
        assert_eq!(request.results[1].result, VerificationState::Approved);

        // the third validator approved it
        contract.report_verification_result(
            valid2.to_string(),
            subject_id.to_string(),
            VerificationState::Approved,
            "".to_string(),
        );
        let request = contract.verifications.get(&subject_id).unwrap();
        assert_eq!(request.results[2].result, VerificationState::Approved);

        // the first validator approved it
        contract.report_verification_result(
            valid0.to_string(),
            subject_id.to_string(),
            VerificationState::Approved,
            "".to_string(),
        );
        let request = contract.verifications.get(&subject_id).unwrap();
        assert_eq!(request.results[0].result, VerificationState::Approved);

        log!("\ntest_report_verification_result: {:?}", request.results);
    }

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
        ))
        .unwrap();

        // make the XCC call
        let promise_idx = env::promise_create(
            contract_id.clone(), // the account id
            method_name,         // the method to call on the contract
            &params,             // the serialized method params
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

    #[test]
    fn test_bind_card_file() {
        // Basic set up for the unit test
        testing_env!(VMContextBuilder::new().build());
        let subject_id = "ar_dni_12488401".to_string();
        let card_id = "Qmc3kQzgwWof7mLG7PPPb1vx6DYDqv9YrW1nWBGhHRqiWW".to_string();

        let mut contract = VerificationContract::new();
        contract.bind_card_file(
          subject_id.to_string(), 
          card_id.to_string());

        log!("\ntest_bind_card_file: subject_id={:?} card_id={:?}", 
          subject_id, card_id
        );
    }

}
