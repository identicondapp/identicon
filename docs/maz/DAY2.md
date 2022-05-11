
# Actividades Día 2 (Miercoles 11-May)

### Inicial

Agregar `cards` a `VerificationContract` y mantener la version anterior como `VerificationContractV1`
~~~rust
    // emmited certification cards for approved subjects
    pub cards: UnorderedMap<SubjectId, FileId>
~~~

Agregar el codigo de migración al contrato:
~~~rust
#[near_bindgen]
impl VerificationContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        log!("\ninit:: initialized contract state v2: verifications, assignments, validators, cards");
        Self {
            // this are Contract v1 props
            verifications: UnorderedMap::new(b"c"),
            assignments: UnorderedMap::new(b"u"),
            validators: Vec::new(),

            // this is the new addition to Contract V2
            cards: UnorderedMap::new(b"v"), 
        }
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        log!("\nmigrate: migrating contract state v1 to v2: verifications, assignments, validators");
        let old_state: VerificationContractV1 = env::state_read().expect("failed");
        Self {
            // this props values must be preserved
            verifications: old_state.verifications,
            assignments: old_state.assignments,
            validators: old_state.validators,

            // now initialize the new 'cards' map
            cards: UnorderedMap::new(b"v"), 
        }
    }
~~~

Correr tests unitarios para verificar que sigue andando todo - OK

### Testing de la migración

Antes de testear la migración:

- agregar cambios al branch actual, la version V2 del contrato `ncar/maz`
- crear un branch con la version V1 del contrato `ncar/maz/contractv1`
- reset de ese branch al commit previo a los cambios y dejar así

Para testear la migración de forma repetitiva: 

1. checkout del branch V1 => tenemos el codigo del contrato V1 
2. build, deploy del contrato, inicializamos, y corremos `runall.sh` para cargarle datos 
3. ya tenemos el estado del contrato con la estructura de V1
4. checkout del branch modificado =>  tenemos el codigo del contrato V2
5. build, deploy del contrato modificado con `near deploy ... --initFunction "migrate" ...`
