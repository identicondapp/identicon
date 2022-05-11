# CHANGELOG

Aquí solo se documentan cambios en:

- el código, 
- la configuración del ambiente 
- herramientas de build y deploy.

La mayor parte de estos cambios están registrados en los mensajes de `commit`,  pero aquí se provee una descripción resumida.

### Día 1 (10-05-2022)

- FLAGS/DEPS: Aplicados cambios en `Cargo.toml` para optimizar generación de codigo.
- REFACTORING: mover definición de tipos, structs y constantes
- REFACTORING: mover metodos del solicitante (request) y sus privates
- REFACTORING: mover metodos de validadores
- REFACTORING: mover payments
- REFACTORING: mover tests
- FIXES: corregir uso de `log!` y `assert!` cuando no fue usado
- CLEANUP: limpiar deps no usadas y warns, aplicar `cargo ftm`

### Día 2 (11-05-2022)

- CAMBIOS: agregar 'cards' a `VerificationContract` 
- CAMBIOS: agregar codigo para migracion
- CAMBIOS: agregar codigo para upgrade desde DAO

