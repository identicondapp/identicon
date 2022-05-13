use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::PanicOnDefault;

// The Subject government identification as a string formed
// using 'type'+'number'+'country', ex: 'dni:12488353:ar'
pub type SubjectId = String;

// The NEAR account who requests the verification
pub type RequestorId = String;

// A NEAR account ID, ex: 'validator1.identicon.near'
pub type ValidatorId = String;

// A DateTime in ISO format 'AAAA-MM-DD hh:mm:ss', ex: '2022-03-27 00:00:00'
pub type ISODateTime = String;

// A Unique ID key (IPFS content-hash) for files stored in Web3.storage or equivalent
pub type FileId = String;

// The location coordinates as obtained from GoogleMaps/other
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GPSCoordinates {
    pub long: String,
    pub lat: String,
}

// A naive implementation for the Subject Contact info
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ContactInfo {
    pub phones: String,
    pub email: String,
}

// A naive implementation for the subject Address location
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct LocationInfo {
    pub directions: String, // ex: 'Calle Las Lomitas Nro. 23 e/ Pampa y La Via'
    pub city: String,
    pub province: String,
    pub country: String, // ex 'mx', 'ar', 've', 'bo', cl', 'uy', ...
    pub coordinates: GPSCoordinates,
}

// The Time Window in which the verification must be performed
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TimeWindow {
    pub starts: ISODateTime,
    pub ends: ISODateTime,
}

// All the relevant Subject information
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SubjectInfo {
    pub age: u8,
    pub sex: String,
    pub contact: ContactInfo,
    pub address: LocationInfo,
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
pub const MIN_VALIDATORS: u8 = 3;
pub const MAX_VALIDATORS: u8 = 4;

// 1 Ⓝ in yoctoNEAR
// to be paid to validator when task is completed
pub const PRIZE_AMOUNT: u128 = 1_000_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VerificationResult {
    pub validator_id: ValidatorId,
    pub result: VerificationState,
    pub timestamp: ISODateTime,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct VerificationRequest {
    // the verification service required, which may include additional info
    // for some types such as ProofOfOwnership(asset) or ProofOfService(service).
    pub is_type: VerificationType,

    // this is the account who requested the verification and will pay for it,
    // and is NOT the same as the subject to be verified.
    pub requestor_id: RequestorId,

    // this is the subject to be verified, which is ALLWAYS a real human being,
    // cats, dogs and other pets may be considered in the future :-)
    pub subject_id: SubjectId,
    pub subject_info: SubjectInfo,
    pub when: TimeWindow,

    // the verification state of the whole request, as a result of the individual
    // verifications. If any of the individual verifications is Rejected, then the
    // whole verification is Rejected.
    pub state: VerificationState,

    // the array [MIN_VALIDATORS..MAX_VALIDATORS] of individual validator verifications
    pub results: Vec<VerificationResult>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Debug)]
pub struct VerificationContract {
    // the pending verifications as a iterable Map keyed by SubjectId
    pub verifications: UnorderedMap<SubjectId, VerificationRequest>,

    // the assigned validations, as a Map keyed by ValidatorId
    // the value is a (variable) list of the SubjectIds to verify
    pub assignments: UnorderedMap<ValidatorId, Vec<SubjectId>>,

    // the Pool of validators, as an array of ValidatorIds
    pub validators: Vec<ValidatorId>,

    // emited certification cards for approved subjects
    pub cards: UnorderedMap<SubjectId, FileId>,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Debug)]
pub struct VerificationContractV1 {
    // the pending verifications as a iterable Map keyed by SubjectId
    pub verifications: UnorderedMap<SubjectId, VerificationRequest>,

    // the assigned validations, as a Map keyed by ValidatorId
    // the value is a (variable) list of the SubjectIds to verify
    pub assignments: UnorderedMap<ValidatorId, Vec<SubjectId>>,

    // the Pool of validators, as an array of ValidatorIds
    pub validators: Vec<ValidatorId>,
}
