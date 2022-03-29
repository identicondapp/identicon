# Identicon protocol

## Objectives

Identicon is a verification protocol for the OpenWeb.

Use cases:

- Proof of Life
- Proof of Identity 
- Proof  of Existence
- Proof of State
- Proof of Ownership
- Proof of Service

## Concepts

#### What is a *verification* ?

#### How is the verification done ?

1. Someone (aka the *Requestor*) requests verification of the identity/life/etc of some particular subject

2. A group of persons  (aka the *Validators*) visit the subject in the given address, within a certain *Time Window*.
3.  

#### What types of verifications are allowed ?

#### Who can *request* a verification ?

Anybody who has a NEAR account can request it.

 #### Who will pay for it ?

The requestor must pay for the verification, and it is allways a NEAR account.

#### What is the *subject* ?

#### What is a government identification ?

#### Who can be a *validator* ?

#### How will they be compensated ?

#### What is a *decentralized ID* ?

#### What is the *digital passport* ?

## Structures

~~~rust

// The Subject government identification as a string formed 
// using 'type'+'number'+'country', ex: 'dni:12488353:ar'
type SubjectId = String;  

// A NEAR account ID, ex: 'juanmescher.near'
type ValidatorId = String; 

// A DateTime in ISO format 'AAAA-MM-DD hh:mm:ss', ex: '2022-03-27 00:00:00'
type ISODateTime = String; 

// The location coordinates as obtained from GoogleMaps/other
struct GPSCoordinates {
	long: u64,
	lat: u64
}

// A naive implementation for the Subject Contact info
struct ContactInfo {
	phones: String,
  email: String,
}

// A naive implementation for the subject Address location
struct Location {
  directions: String, // ex: 'Calle Las Lomitas Nro. 23 e/ Pampa y La Via'
  city: String,
  province: String,
  country: String, // ex 'mx', 'ar', 've', 'bo', cl', 'uy', ...
  coordinates: GPSCoordinates
}

// The Time Window in which the verification must be performed
struct TimeWindow {
	starts: ISODateTime,
  ends: ISODateTime
}

// The different verification services 
enum VerificationType {
  /// Validates that the Subject is alive, and lives in the indicated Location.
  /// It also implies a ProofOfIdentity. This is a recurrent validation, 
  // meaning it must be repeated every month.
	ProofOfLife,
  
  /// Validates that the Subject is who he says he is, and that is (obviously) alive.
  ProofOfIdentity,

  // Not implemented, reserved for future use
  ProofOfExistence { asset: String },
  ProofOfState { asset: String },
  ProofOfOwnership { asset: String },
  ProofOfService { service: String },
}

enum VerificationState {
  /// Started but waiting for the validator results  
  Pending, // code: P

  /// Verification result is approved
  Approved, // code: AP

  /// Verification result is Rejected
  Rejected { why: String }, // code: RX

  /// It is not possible to do the verification, due to some reason which exceeds 
  /// the Validator possibilites, such as inaccesible area, weather, etc
  NotPossible { why: String }, // code: NP

  /// Validator will not do the verification, for some personal reason,
  /// but it requires a cause and explanation. Too many of this refusals 
  /// may eliminate the Validator from the validators pool.
  WillNotDo { why: String } // code: WND
  
  /// Verification was canceled by Requestor
  Canceled { why: String } // code: CX
}

// The min and max required validators to verify a given request
// it may vary randomly between MIN and MAX
const MIN_VALIDATORS = 3;
const MAX_VALIDATORS = 4;

struct VerificationResult {
	validator_id: ValidatorId,
  state: VerificationState
}

struct VerificationRequest {
  // the verification service required, which may include additional info
  // for some types such as ProofOfOwnership(asset) or ProofOfService(service).
  is_type: VerificationType,
  
  // this is the account who requested the verification and will pay for it,
  // and is NOT the same as the subject to be verified.
  requestor_id: AccountId,
  
  // this is the subject to be verified, which is ALLWAYS a real human being,
  // cats, dogs and other pets may be considered in the future :-)
  subject_id: SubjectId,
  contact: ContactInfo,
  where: LocationInfo,
  when: TimeWindow,
  
  // the verification state of the whole request, as a result of the individual
  // verifications. If any of the individual verifications is Rejected, then the
  // whole verification is Rejected.
  state: VerificationState, 
  
  // the array [MIN_VALIDATORS..MAX_VALIDATORS] of individual validator verifications  
  results: Vec<VerificationResult> 
}

pub struct VerificationContract {
  // the pending verifications as a iterable Map keyed by SubjectId
  verifications: UnorderedMap<SubjectId, VerificationRequest>,
  
  // the assigned validations, as a Map keyed by ValidatorId
  // the value is a (variable) list of the SubjectIds to verify
	assignments: UnorderedMap<ValidatorId, Vec<SubjectId>>,
  
  // the Pool of validators, as an array of ValidatorIds
  validators: Vec<ValidatorId>,
}

~~~

## Methods

### Called by *Requestor*

- `request_verification(is_type, subject_id, contact, where, when)` Registers the new request in the blockchain and assigns validators to verify it.

- `cancel_verification(subject_id, cause)` Not implemented

- `get_verification_transactions(subject_id)` Not implemented

- `get_requested_verifications_history(filters)` Not implemented

- `mint_digital_passport(subject_id)`  Not implemented

### Called by *Validators*

- `get_assigned_verifications(self)` Not implemented

- `report_verification_result(self, subject_id, result, cause)` IMPLEMENT

- `register_as_validator(self)` IMPLEMENT

- `unregister_as_validator(self)` Not implemented
- `get_my_verifications_history(self, filters)` Not implemented

### Private

- `assign_validators(subject_id)` IMPLEMENT
- `evaluate_request_state(results) -> VerificationState`   IMPLEMENT
- `pay_validators(validators)`  WILL TRY, NOT SURE WE WILL IMPLEMENT !

