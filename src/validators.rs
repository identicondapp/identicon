use near_sdk::log;
use near_sdk::near_bindgen;

use crate::definitions::*;

#[near_bindgen]
impl VerificationContract {
    // Report the result of the verification. If the verification was not possible,
    // or the validator will not do it then  the validator must include a
    // descriptive cause.
    /* Called by *Validators* */
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
            } else {
                changed.push(before.clone());
            }
        }

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

    // Every time we receive a verification result we must evaluate if all verifications have been done,
    // and which is the final result for the request. While the verifications are still in course the
    // request state is Pending.
    fn evaluate_results(&mut self, results: Vec<VerificationResult>) -> VerificationState {
        // first check if we have some pending result
        if results
            .iter()
            .any(|e| e.result == VerificationState::Pending)
        {
            return VerificationState::Pending;
        }

        // now check if we have some of it rejected
        match results
            .iter()
            .find(|e| e.result == VerificationState::Rejected)
        {
            Some(_) => return VerificationState::Rejected,
            None => {}
        }

        // check if we have the min required approvals
        let approvals: u8 = results.iter().fold(0, |count, _| count + 1);
        if approvals >= MIN_VALIDATORS {
            return VerificationState::Approved;
        }

        VerificationState::Pending
    }
}
