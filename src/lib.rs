use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::log;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};
use near_sdk::{AccountId, PanicOnDefault, Promise};

// The Subject government identification as a string formed
// using 'type'+'number'+'country', ex: 'dni:12488353:ar'
type SubjectId = String;

// The NEAR account who requests the verification
type RequestorId = String;

// A NEAR account ID, ex: 'validator1.identicon.near'
type ValidatorId = String;

// A DateTime in ISO format 'AAAA-MM-DD hh:mm:ss', ex: '2022-03-27 00:00:00'
type ISODateTime = String;

// The location coordinates as obtained from GoogleMaps/other
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
struct GPSCoordinates {
    long: String,
    lat: String,
}

// A naive implementation for the Subject Contact info
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
struct ContactInfo {
    phones: String,
    email: String,
}

// A naive implementation for the subject Address location
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
struct LocationInfo {
    directions: String, // ex: 'Calle Las Lomitas Nro. 23 e/ Pampa y La Via'
    city: String,
    province: String,
    country: String, // ex 'mx', 'ar', 've', 'bo', cl', 'uy', ...
    coordinates: GPSCoordinates,
}

// The Time Window in which the verification must be performed
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
struct TimeWindow {
    starts: ISODateTime,
    ends: ISODateTime,
}

// All the relevant Subject information
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SubjectInfo {
    age: u8,
    sex: String,
    contact: ContactInfo,
    address: LocationInfo,
}

// The different verification services
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum VerificationType {
    /// Validates that the Subject is alive, and lives in the indicated Location.
    /// It also implies a ProofOfIdentity. This is a recurrent validation,
    // meaning it must be repeated every month.
    ProofOfLife,

    /// Validates that the Subject is who he says he is, and that is (obviously) alive.
    ProofOfIdentity,

    // Not implemented, reserved for future use
    ProofOfExistence {
        asset: String,
    },
    ProofOfState {
        asset: String,
    },
    ProofOfOwnership {
        asset: String,
    },
    ProofOfService {
        service: String,
    },
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum VerificationState {
    /// Started but waiting for the validator results  
    Pending, // code: P

    /// Verification result is approved
    Approved, // code: AP

    /// Verification result is Rejected
    Rejected, // code: RX

    /// It is not possible to do the verification, due to some reason which exceeds
    /// the Validator possibilites, such as inaccesible area, weather, etc
    NotPossible { why: String }, // code: NP

    /// Validator will not do the verification, for some personal reason,
    /// but it requires a cause and explanation. Too many of this refusals
    /// may eliminate the Validator from the validators pool.
    WillNotDo { why: String }, // code: WND

    /// Verification was canceled by Requestor
    Canceled { why: String }, // code: CX
}

// The min and max required validators to verify a given request
// it may vary randomly between MIN and MAX
const MIN_VALIDATORS: u8 = 3;
const MAX_VALIDATORS: u8 = 4;

// 1 Ⓝ in yoctoNEAR
// to be paid to validator when task is completed
const PRIZE_AMOUNT: u128 = 1_000_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
struct VerificationResult {
    validator_id: ValidatorId,
    result: VerificationState,
    timestamp: ISODateTime,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
struct VerificationRequest {
    // the verification service required, which may include additional info
    // for some types such as ProofOfOwnership(asset) or ProofOfService(service).
    is_type: VerificationType,

    // this is the account who requested the verification and will pay for it,
    // and is NOT the same as the subject to be verified.
    requestor_id: RequestorId,

    // this is the subject to be verified, which is ALLWAYS a real human being,
    // cats, dogs and other pets may be considered in the future :-)
    subject_id: SubjectId,
    subject_info: SubjectInfo,
    when: TimeWindow,

    // the verification state of the whole request, as a result of the individual
    // verifications. If any of the individual verifications is Rejected, then the
    // whole verification is Rejected.
    state: VerificationState,

    // the array [MIN_VALIDATORS..MAX_VALIDATORS] of individual validator verifications
    results: Vec<VerificationResult>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Debug)]
pub struct VerificationContract {
    // the pending verifications as a iterable Map keyed by SubjectId
    verifications: UnorderedMap<SubjectId, VerificationRequest>,

    // the assigned validations, as a Map keyed by ValidatorId
    // the value is a (variable) list of the SubjectIds to verify
    assignments: UnorderedMap<ValidatorId, Vec<SubjectId>>,

    // the Pool of validators, as an array of ValidatorIds
    validators: Vec<ValidatorId>,
}

#[near_bindgen]
impl VerificationContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            verifications: UnorderedMap::new(b"c"),
            assignments: UnorderedMap::new(b"u"),
            validators: Vec::new(),
        }
    }

    /* Called by *Requestor* */

    // Registers the new request in the blockchain and assigns validators to verify it.
    pub fn request_verification(
        &mut self,
        requestor_id: RequestorId,
        is_type: VerificationType,
        subject_id: SubjectId,
        subject_info: SubjectInfo,
    ) {
        log!(
            "\nrequest_verification: Called method request_verification({:?} {:?} {:?})",
            requestor_id,
            is_type,
            subject_id
        );

        // check if subject_id already exists in verifications
        assert!(
            !self
                .verifications
                .keys_as_vector()
                .iter()
                .any(|e| e == subject_id),
            "request_verification: Verification already exists for subject_id"
        );

        let mut request = VerificationRequest {
            is_type: is_type,
            requestor_id: requestor_id.to_string(),
            subject_id: subject_id.to_string(),
            subject_info: subject_info,
            when: TimeWindow {
                starts: "2022-03-28 00:00:00".to_string(),
                ends: "2022-03-31 15:00:00".to_string(),
            },
            state: VerificationState::Pending,
            results: Vec::new(),
        };

        // randomly assign the validators
        let selected_validators: Vec<ValidatorId> = self.assign_validators(subject_id.to_string());
        log!(
            "request_verification: Assign selected validators {:?}",
            selected_validators
        );

        for validator in selected_validators.iter() {
            // add to the request results Vec
            request.results.push(VerificationResult {
                validator_id: validator.to_string(),
                result: VerificationState::Pending,
                timestamp: "".to_string(),
            });

            self.add_to_assignments(validator.to_string(), subject_id.to_string());
        }

        // add this request to the verifications to do
        self.verifications.insert(&subject_id.to_string(), &request);
        log!(
            "request_verification: Added to verifications list {:?}",
            &request
        );
    }

    /// Adds this subject to the validator assignments
    fn add_to_assignments(&mut self, validator_id: ValidatorId, subject_id: SubjectId) {
        let existent = self.assignments.get(&validator_id.to_string());
        let mut assigned = if existent.is_some() {
            existent.unwrap()
        } else {
            Vec::new()
        };
        assigned.push(subject_id.to_string());
        self.assignments
            .insert(&validator_id.to_string(), &assigned);
        log!(
            "request_verification: Assigned subject {:?} to validator {:?}",
            subject_id,
            validator_id
        );
    }


    // After reception of all the validators results, we must pay each of the validators 
    // the corresponding compensation (1 NEAR). Validators which did not complete 
    // the verification (for whatever reason) will not receive payment.
    pub fn pay_validators(&mut self, requestor_id: RequestorId, subject_id: SubjectId) {
        log!(
            "\npay_validators: Called method pay_validators({:?} {:?})",
            requestor_id,
            subject_id
        );

        // check if subject_id exists in verifications
        assert!(
            self.verifications
                .keys_as_vector()
                .iter()
                .any(|e| e == subject_id),
            "pay_validators: Verification not found for subject_id"
        );

        let verification = self.verifications.get(&subject_id).unwrap();
        log!(
            "pay_validators: Verification found for subject_id {:?} with state: {:?}",
            subject_id,
            verification.state
        );

        // Valid payable states
        let payable_states = vec![VerificationState::Approved, VerificationState::Rejected];
        for result in verification.results.iter() {
            // Check if result state is payable and should be paid
            if payable_states.iter().any(|e| e == &result.result) {
                log!(
                    "pay_validators: Payable validator found {:?}",
                    result.validator_id
                );

                // Now, we pay

                // 1. Ensure there's enough balance to pay this out
                if env::account_balance() < PRIZE_AMOUNT {
                    log!("The smart contract does not have enough balance to pay this out. :/");
                    continue;
                }

                // 2. Transfer the prize
                let validator: AccountId = result.validator_id.parse().unwrap();
                Promise::new(validator).transfer(PRIZE_AMOUNT);
            }
        }
    }

    
    /* Called by *Validators* */

    // Report the result of the verification. If the verification was not possible,
    // or the validator will not do it then  the validator must include a 
    // descriptive cause.
    pub fn report_verification_result(
        &mut self,
        validator_id: ValidatorId,
        subject_id: SubjectId,
        stated: VerificationState,
        cause: String,
    ) {
        log!(
            "\nreport_verification_result: Called method ({:?} {:?} {:?})",
            validator_id,
            subject_id,
            stated
        );

        // check if subject_id exists in verifications
        assert!(
            self.verifications
                .keys_as_vector()
                .iter()
                .any(|e| e == subject_id),
            "report_verification_result: Request not found for subject_id"
        );

        let mut requested = self.verifications.get(&subject_id).unwrap();
        
        let mut changed: Vec<VerificationResult> = Vec::new();
        for before in requested.results.iter() { 
            if before.validator_id == validator_id { 
                changed.push(VerificationResult {
                    validator_id: validator_id.to_string(),
                    result: stated.clone(),
                    timestamp: "2022-03-31 16:00:00".to_string(),
                });
            } 
            else {
                changed.push(before.clone());
            }
        };

        // and update the full request state
        requested.results = changed.clone();
        self.verifications.insert(&subject_id, &requested);
    }


    /// Some NEAR account owner registers itself as a validator.
    pub fn register_as_validator(&mut self, validator_id: ValidatorId) {
        log!("{:?}", validator_id);
        self.validators.push(validator_id)
    }

    pub fn get_validators_count(&self) -> usize {
        self.validators.len()
    }


    /* Private */

    // When the request is filled, we must select a number of validators at random from the validators pool, and assign them to the request
    fn assign_validators(&self, subject_id: SubjectId) -> Vec<ValidatorId> {
        //Vec::new()
        let val1: ValidatorId = self.validators[1].to_string();
        let val2: ValidatorId = self.validators[2].to_string();
        let val3: ValidatorId = self.validators[3].to_string();
        vec![val1, val2, val3]
    }


    // Every time we receive a verification result we must evaluate if all verifications have been done, and which is the final result for the request. While the verifications are still in course the request state is Pending.
    fn evaluate_results(&mut self, results: Vec<VerificationResult>) -> VerificationState {
        // first check if we have some pending result
        if results.iter().any(|e| e.result == VerificationState::Pending) {
            return VerificationState::Pending;
        }

        // now check if we have some of it rejected
        match results.iter().find(|e| e.result == VerificationState::Rejected) {
            Some(_) => { return VerificationState::Rejected },
            None => { }
        }

        // check if we have the min required approvals
        let approvals: u8 = results.iter().fold(0, |count, _| count + 1);
        if approvals >= MIN_VALIDATORS {
            return VerificationState::Approved
        }

        VerificationState::Pending
    }

    /* Not implemented */

    /* TODO: to be implemented */

    //cancel_verification(subject_id, cause)

    //get_verification_transactions(requestor_id, subject_id)

    //get_all_verifications_history(requestor_id, filters)

    //mint_digital_passport(requestor_id, subject_id)

    //unregister_as_validator(validator_id, self)

    //get_my_assigned_verifications(validator_id)

    //get_my_verifications_history(validator_id, filters)
}

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

        contract.request_verification(
            requestor_id.to_string(),
            VerificationType::ProofOfLife,
            subject_id.to_string(),
            moq_subject_info(),
        );

        let request = contract.verifications.get(&subject_id).unwrap();
        assert_eq!(request.requestor_id, requestor_id);
        assert_eq!(request.subject_id, subject_id);
        assert_eq!(request.results.len(), 2);
        assert_eq!(contract.verifications.len(), 1);
        assert_eq!(contract.assignments.len(), 2);
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
            "".to_string());
        let request = contract.verifications.get(&subject_id).unwrap();
        assert_eq!(request.results[1].result, VerificationState::Approved);

        // the third validator approved it
        contract.report_verification_result(
            valid2.to_string(), 
            subject_id.to_string(),
            VerificationState::Approved,
            "".to_string());
        let request = contract.verifications.get(&subject_id).unwrap();
        assert_eq!(request.results[2].result, VerificationState::Approved);

        // the first validator approved it
        contract.report_verification_result(
            valid0.to_string(), 
            subject_id.to_string(),
            VerificationState::Approved,
            "".to_string());
        let request = contract.verifications.get(&subject_id).unwrap();
        assert_eq!(request.results[0].result, VerificationState::Approved);

        log!("\ntest_report_verification_result: {:?}", request.results);
    }

}
