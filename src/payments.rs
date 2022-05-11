use near_sdk::log;
use near_sdk::{env, near_bindgen};
use near_sdk::{AccountId, Promise};

use crate::definitions::*;

#[near_bindgen]
impl VerificationContract {
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
}
