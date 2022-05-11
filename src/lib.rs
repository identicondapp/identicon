use near_sdk::collections::UnorderedMap;
use near_sdk::log;
use near_sdk::{env, near_bindgen};

mod definitions;
mod payments;
mod requests;
mod validators;
use definitions::*;

#[cfg(test)]
mod tests;

#[near_bindgen]
impl VerificationContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        log!("\ninitialized contract state: [verifications], [assignments], [validators]");
        Self {
            verifications: UnorderedMap::new(b"c"),
            assignments: UnorderedMap::new(b"u"),
            validators: Vec::new(),
        }
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
