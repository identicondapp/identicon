# NCAR - Bootcamp 09-05-2022 al 13-05-2022

### Día 1 (Martes 10-May)

**A realizar**

- `HECHO` Clona el repositorio de Github, compila y despliega el contrato. 
- `HECHO` Crea la estructura de archivos para tu contrato inteligente, es decir, los archivos migrate.rs, internals.rs, enumerations.rs y los que consideres necesarios.
- `HECHO` Implementa las buenas prácticas recomendadas por el Protocolo de NEAR para el lenguaje de programación Rust. 
- `HECHO` Corrige el archivo **Cargo.toml** para optimizar el peso del archivo compilado.
- `HECHO` ¡Compila y Despliega tu contrato para realizar las pruebas necesarias y seguir añadiendo las herramientas para escalabilidad y mantenimiento para tu DApp!

**Realizado en**: [Día 1](./DAY1.md)

### Día 2 (Miercoles 11-May)

NOTA: se adelantan aquí las actividades pensadas para el tercer día, por estra más relacionadas con el codigo revisado.

**A realizar**

- `HECHO` Añade un cambio en la estructura de tu contrato e implementa el método migrate.
- `HECHO` Despliega el contrato con los cambios realizados con el comando llamando como función inicial al método migrate.
- `HECHO` Crea una DAO en [SputnikDAO 2](https://testnet-v2.sputnik.fund/#/), realiza cambios en el contrato y haz una propuesta para actualizar un contrato. 

**Realizado en**: [Día 2](./DAY2.md)

### Día 3 (Jueves 12-May)

- La primer actividad del día será crear un subgrafo en The Graph, realizar su despliegue, su configuración, y el mapeo necesarios para leer información del [NCAR-Example](https://github.com/open-web-academy/NCAR-Example) con las modificaciones realizadas el día de ayer.
    - La manera de montar un subgrafo es algo extensa, así que, aquí te dejamos un tutorial de cómo realizarlo
        
        [Creación de un nodo en The Graph](https://www.notion.so/Creaci-n-de-un-nodo-en-The-Graph-9ac3f2c4745c479c8c629901e580439b)
        
- Después de montar el subgrafo, elige cualquier servidor **IPFS** para subir un contenido digital, obtener un hash único y añadirlo como dato en la estructura de tu contrato ejemplo.
- La última actividad consta de un trabajo en equipo o individual, en este caso se crearán los métodos para realizar un Cross Contract Callback, puede ser implementado todo en el mismo contrato o se trabajará a la par con alguien más y una parte del código será implementado en el contrato que realiza la llamada y la otra será en el contrato que reciba la llamada.

