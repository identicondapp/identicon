use near_sdk::near_bindgen;
use near_sdk::{env, log};
use serde_json::json;

use crate::definitions::*;

#[near_bindgen]
impl VerificationContract {
    // Registers the new request in the blockchain and assigns validators to verify it,
    // is called by *Requester*
    pub fn request_verification(
        &mut self,
        requestor_id: RequestorId,
        is_type: VerificationType,
        subject_id: SubjectId,
        subject_info: SubjectInfo,
    ) {
        // log #0 as JSON for TheGraph
        env::log_str(
            &json!({
                "type": VerificationType::ProofOfLife,
                "requestor_id": requestor_id.to_string(),
                "subject_id": subject_id.to_string(),
                "state": VerificationState::Pending
            })
            .to_string(),
        );

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

        // return the verification state
        request.state;
    }

    /* Private */

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

    // When the request is filled, we must select a number of validators at random from the validators pool, and assign them to the request
    fn assign_validators(&self, subject_id: SubjectId) -> Vec<ValidatorId> {
        //Vec::new()
        let val1: ValidatorId = self.validators[1].to_string();
        let val2: ValidatorId = self.validators[2].to_string();
        let val3: ValidatorId = self.validators[3].to_string();
        vec![val1, val2, val3]
    }
}
