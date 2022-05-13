use near_sdk::near_bindgen;
use near_sdk::{env, log};
use serde_json::json;

use crate::definitions::*;

#[near_bindgen]
impl VerificationContract {
    pub fn bind_card_file(&mut self, subject_id: SubjectId, card_id: FileId) {
        log!(
            "\nbind_card_file: Called method bind_card_file({:?} {:?})",
            subject_id,
            card_id
        );
        
        self.cards
            .insert(&subject_id.to_string(), &card_id.to_string());

        // return the changed [subject_id, CID]
        vec![&subject_id.to_string(), &card_id.to_string()];
    }
}
