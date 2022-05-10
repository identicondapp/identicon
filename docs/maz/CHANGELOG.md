# CHANGELOG

Aquí solo se documentan cambios en:

- el código, 
- la configuración del ambiente 
- herramientas de build y deploy.

La mayor parte de estos cambios están registrados en los mensajes de `commit`,  pero aquí se provee una descripción más detallada.

### Día 1

- Aplicados cambios en `Cargo.toml` para optimizar generación de codigo.
- REFACTORING: Definición de tipos, structs y constantes pasa a `definitions.rs`
- REFACTORING: `request_verification` y sus privates pasan a `requests.rs`
