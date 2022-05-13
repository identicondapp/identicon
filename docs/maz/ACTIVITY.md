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

- `HECHO` La primer actividad del día será crear un subgrafo en The Graph, realizar su despliegue, su configuración, y el mapeo necesarios para leer información del [NCAR-Example](https://github.com/open-web-academy/NCAR-Example) con las modificaciones realizadas el día de ayer.
    - La manera de montar un subgrafo es algo extensa, así que, aquí te dejamos un tutorial de cómo realizarlo
        
        [Creación de un nodo en The Graph](https://www.notion.so/Creaci-n-de-un-nodo-en-The-Graph-9ac3f2c4745c479c8c629901e580439b)
        
- `HECHO` Después de montar el subgrafo, elige cualquier servidor **IPFS** para subir un contenido digital, obtener un hash único y añadirlo como dato en la estructura de tu contrato ejemplo.
- `HECHO` La última actividad consta de un trabajo en equipo o individual, en este caso se crearán los métodos para realizar un Cross Contract Callback, puede ser implementado todo en el mismo contrato o se trabajará a la par con alguien más y una parte del código será implementado en el contrato que realiza la llamada y la otra será en el contrato que reciba la llamada.

**Realizado en**: [Día 3](./DAY3.md) y [Día 4](./DAY4.md)

### Día 4 (Viernes 13-May)

Configuramos ambiente para `mainnet`, nos logeamos con `marizozito.near` y llamamos a `nativo-nft
~~~
$ export NEAR_ENV=mainnet
$ near login

Please authorize NEAR CLI on at least one of your accounts.

If your browser doesn't automatically open, please visit this URL
https://wallet.near.org/login/?referrer=NEAR+CLI&public_key=ed25519%3A5JNn9YR5pFPDUu2x4izUZCw82VJgjS1cdgZqQ2JBekxa&success_url=http%3A%2F%2F127.0.0.1%3A5000
Please authorize at least one account at the URL above.

Which account did you authorize for use with NEAR CLI?
Enter it here (if not redirected automatically):
Logged in as [ mariozito.near ] with public key [ ed25519:5JNn9Y... ] successfully
~~~

LLamamos a `nativo-nft

LLamamos a `paras`
~~~
$ near call x.paras.near nft_token '{"token_id":"302102:1"}' --accountId mariozito.near
Scheduling a call: x.paras.near.nft_token({"token_id":"302102:1"})
Doing account.functionCall()
Transaction Id 7RxoimizyfwHK6cWGZ1r7r7gG5cxdDZmeD86bLULzi4x
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.mainnet.near.org/transactions/7RxoimizyfwHK6cWGZ1r7r7gG5cxdDZmeD86bLULzi4x
{
  token_id: '302102:1',
  owner_id: 'joehank.near',
  metadata: {
    title: 'SPACE NEAR #237 #1',
    description: null,
    media: 'bafybeiglpurvi45bmb6ldquaxwo7ke6e53qvyivfb5j2jqtd3gdrku6qei',
    media_hash: null,
    copies: 1,
    issued_at: '1647557881424333278',
    expires_at: null,
    starts_at: null,
    updated_at: null,
    extra: null,
    reference: 'bafkreifeeroe62rlsfpqofynju436425sitdkiepvvsn47c4o2hlbgdifa',
    reference_hash: null
  },
  approved_account_ids: {}
}
~~~

LLamamos a `nativo-nft`
~~~
$ near call nativo-minter.near nft_token '{"token_id":"5"}' --accountId mariozito.near
Scheduling a call: nativo-minter.near.nft_token({"token_id":"5"})
Doing account.functionCall()
Transaction Id 6hNEu29igcJxDbvojKTGkpN4A67itMUoUdfDnrzCwcw5
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.mainnet.near.org/transactions/6hNEu29igcJxDbvojKTGkpN4A67itMUoUdfDnrzCwcw5
{
  token_id: '5',
  owner_id: 'kitachi5258.near',
  metadata: {
    title: 'Muñequitas Étnicas 001',
    description: 'Muñequita étnica inspirada en la cultura Wixárika',
    media: 'bafybeibvs7eborzjjht5v6bnmq7oqixmjjqv6ftkhpmj636zw4t34higwy',
    media_hash: null,
    copies: null,
    issued_at: null,
    expires_at: null,
    starts_at: null,
    updated_at: null,
    extra: null,
    reference: null,
    reference_hash: null
  },
  creator_id: 'kitachi5258.near',
  approved_account_ids: {},
  royalty: { 'kitachi5258.near': 1000 }
}
~~~
