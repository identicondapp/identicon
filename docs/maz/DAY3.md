
# Actividades Día 3 (Jueves 12-May)

### TheGraph

https://thegraph.com/hosted-service/dashboard

https://ow-academy.notion.site/Creaci-n-de-un-nodo-en-The-Graph-9ac3f2c4745c479c8c629901e580439b#6f233461323f41bdac95bffca91426b0

https://github.com/open-web-academy/thegraph/blob/main/src/mapping.ts

https://github.com/open-web-academy/NCAR-Example/blob/main/src/lib.rs

Access token

```
ca86f3cf97ea44a0a9f1d1cea3ff1ade
```

Instalar 

~~~sh
sudo yarn global add @graphprotocol/graph-cli
~~~

NOTA: se cuelga con NPM. Usamos `yarn`y todo ok.

NOTA: instalar con `sudo` para que puede ser accedido global.

Luego:

~~~
$ cd ~/dev/learn/near

$ graph init --product hosted-service mazito/maz-ncar052022
✔ Protocol · near
✔ Subgraph name · mazito/maz-ncar052022
✔ Directory to create the subgraph in · thegraph
✔ NEAR network · near-testnet
✔ Contract account · identicon.testnet
✔ Contract Name · contract_v2
———
  Generate subgraph
  Write subgraph to directory
✔ Create subgraph scaffold
✔ Initialize networks config
✔ Initialize subgraph repository
✔ Install dependencies with yarn
✔ Generate ABI and schema types with yarn codegen

Subgraph mazito/maz-ncar052022 created in thegraph

Next steps:

  1. Run `graph auth` to authenticate with your deploy key.

  2. Type `cd thegraph` to enter the subgraph.

  3. Run `yarn deploy` to deploy the subgraph.

Make sure to visit the documentation on https://thegraph.com/docs/ for further information.
~~~

~~~
$ cd thegraph/

$ yarn install
yarn install v1.22.15
[1/4] Resolving packages...
success Already up-to-date.
Done in 1.11s.

$ yarn codegen
yarn run v1.22.15
$ graph codegen
  Skip migration: Bump mapping apiVersion from 0.0.1 to 0.0.2
  Skip migration: Bump mapping apiVersion from 0.0.2 to 0.0.3
  Skip migration: Bump mapping apiVersion from 0.0.3 to 0.0.4
  Skip migration: Bump mapping apiVersion from 0.0.4 to 0.0.5
  Skip migration: Bump mapping specVersion from 0.0.1 to 0.0.2
✔ Apply migrations
✔ Load subgraph from subgraph.yaml
✔ Generate types for data source templates
✔ Load GraphQL schema from schema.graphql
  Write types to generated/schema.ts
✔ Generate types for GraphQL schema

Types generated successfully

Done in 1.48s.
~~~

Autorizar

~~~
$ graph auth --product hosted-service ca86f3cf97ea44a0a9f1d1cea3ff1ade
Deploy key set for https://api.thegraph.com/deploy/
~~~

Buscar bloque inicial 

- Transaction: https://explorer.testnet.near.org/blocks/5QWEAkLBqfdGwF4EdrpKQxFNR5nB6sGREhPbz73dDDyi

Editar `subgraph.yaml` 

~~~yaml
specVersion: 0.0.2
schema:
  file: ./schema.graphql
dataSources:
  - kind: near
    name: contract_v2
    network: near-testnet
    source:
      account: "identicon.testnet"
      startBlock: 89726252
    mapping:
      apiVersion: 0.0.6
      language: wasm/assemblyscript
      entities:
        - ExampleEntity
      receiptHandlers:
        - handler: handleReceipt
      file: ./src/mapping.ts
~~~

Deploy del subgrafo `OK`

~~~bash
$ graph deploy --product hosted-service mazito/maz-ncar052022
  Skip migration: Bump mapping apiVersion from 0.0.1 to 0.0.2
  Skip migration: Bump mapping apiVersion from 0.0.2 to 0.0.3
  Skip migration: Bump mapping apiVersion from 0.0.3 to 0.0.4
  Skip migration: Bump mapping apiVersion from 0.0.4 to 0.0.5
  Skip migration: Bump mapping specVersion from 0.0.1 to 0.0.2
✔ Apply migrations
✔ Load subgraph from subgraph.yaml
  Compile data source: contract_v2 => build/contract_v2/contract_v2.wasm
✔ Compile subgraph
  Copy schema file build/schema.graphql
  Write subgraph manifest build/subgraph.yaml
✔ Write compiled subgraph to build/
  Add file to IPFS build/schema.graphql
                .. QmezRTiPrJWoXtoUPq63tbQK5Cg8jZ21C2TFp2eQVJ2jhL
  Add file to IPFS build/contract_v2/contract_v2.wasm
                .. QmRsDhGo2dn3XrS2ZQgcoSwzspFFkCNkzj4rpuwCrWAa69
✔ Upload subgraph to IPFS

Build completed: QmaRv4PTLZVwDCFmjdAychn5PNz7zQrKwKUqp7hz96AXv5

Deployed to https://thegraph.com/explorer/subgraph/mazito/maz-ncar052022

Subgraph endpoints:
Queries (HTTP):     https://api.thegraph.com/subgraphs/name/mazito/maz-ncar052022
Subscriptions (WS): wss://api.thegraph.com/subgraphs/name/mazito/maz-ncar052022

~~~

Subgraph endpoints

- Queries (HTTP): https://api.thegraph.com/subgraphs/name/mazito/maz-ncar052022
- Subscriptions (WS): wss://api.thegraph.com/subgraphs/name/mazito/maz-ncar052022

**Configurar el subgrafo**

Editar schema `schema.graphql` para `contract_v2`

~~~
type VerificationRequest @entity {
  id: ID!
  type: String!
  requestor_id: String!
  subject_id: String!
  state: String!
}

~~~

Codegen

~~~
$ yarn codegen
yarn run v1.22.15
$ graph codegen
  Skip migration: Bump mapping apiVersion from 0.0.1 to 0.0.2
  Skip migration: Bump mapping apiVersion from 0.0.2 to 0.0.3
  Skip migration: Bump mapping apiVersion from 0.0.3 to 0.0.4
  Skip migration: Bump mapping apiVersion from 0.0.4 to 0.0.5
  Skip migration: Bump mapping specVersion from 0.0.1 to 0.0.2
✔ Apply migrations
✔ Load subgraph from subgraph.yaml
✔ Generate types for data source templates
✔ Load GraphQL schema from schema.graphql
  Write types to generated/schema.ts
✔ Generate types for GraphQL schema

Types generated successfully

Done in 1.60s.
~~~

Editar mappings `src/mappings.ts` 

Basado en ejemplo: https://github.com/open-web-academy/thegraph/blob/main/src/mapping.ts

~~~
import { near, BigInt, json, JSONValueKind, log } from "@graphprotocol/graph-ts"
import { VerificationRequest } from "../generated/schema"

export function handleReceipt(receiptWithOutcome: near.ReceiptWithOutcome): void {

  const actions = receiptWithOutcome.receipt.actions;
  
  for (let j=0; j < actions.length; j++) {
    handleReceiptAction(
      actions[j], 
      receiptWithOutcome.receipt, 
      receiptWithOutcome.block.header,
      receiptWithOutcome.outcome,
      receiptWithOutcome.receipt.signerPublicKey
    )
  }
}

const LISTEN_TO = 'request_verification';

function handleReceiptAction(
  action: near.ActionValue,
  receipt: near.ActionReceipt,
  blockHeader: near.BlockHeader,
  outcome: near.ExecutionOutcome,
  publicKey: near.PublicKey
): void {

  // check if its one of the function calls we are listening
  if (action.kind !== near.ActionKind.FUNCTION_CALL) {
    log.info("handleReceiptAction: {}", ["Not a function call"]);
    return;
  }

  if (action.toFunctionCall().methodName !== LISTEN_TO) {
    log.info("handleReceiptAction: {}", ["Not listening for this method"]);
    return;
  }

  log.info("handleReceiptAction: received {} function call", [LISTEN_TO]); 

  // use only the first log line
  let parsed = json.fromString(outcome.logs[0]); 
  if (parsed.kind !== JSONValueKind.OBJECT) {
    log.info("handleReceiptAction: parsed {} not a valid Object", [outcome.logs[0]]); 
    return;
  }
  
  // create the Entity to be stored in the Subgraph
  // Note: If a handler doesn't require existing field values, it is faster
  // _not_ to load the entity from the store. Instead, create it fresh with
  // `new Entity(...)`, set the fields that should be updated and save the
  // entity back to the store. Fields that were not set or unset remain
  // unchanged, allowing for partial updates to be applied.

  // Entity fields can be set based on receipt information
  // we use the receipt ID as the Entity unique key
  let entity = new VerificationRequest(receipt.id.toString());
  
  // Navigate the JSON and copy to the Entity attrs
  // MAZ: no entiendo bien porque se hace asi y no se puede usar la asignacion directa 
  // de props del objeto a la entidad, es porque es AssemblyScript ?
  // afanado del ejemplo (https://github.com/open-web-academy/thegraph/blob/main/src/mapping.ts)
  const entry = parsed.toObject();
  for (let i = 0;i < entry.entries.length; i++) {
    let key = entry.entries[i].key.toString()
    switch (true) {
      case key == 'type':
        entity.type = entry.entries[i].value.toString()
        break
      case key == 'requestor_id':
        entity.requestor_id = entry.entries[i].value.toString()
        break
      case key == 'subject_id':
        entity.subject_id = entry.entries[i].value.toString()
        break
      case key == 'state':
        entity.state = entry.entries[i].value.toString()
        break
    }
  }    

  // Entities can be written to the store with `.save()`
  entity.save();
}

~~~

Redeploy 

~~~
$ yarn codegen
yarn run v1.22.15
$ graph codegen
  Skip migration: Bump mapping apiVersion from 0.0.1 to 0.0.2
  Skip migration: Bump mapping apiVersion from 0.0.2 to 0.0.3
  Skip migration: Bump mapping apiVersion from 0.0.3 to 0.0.4
  Skip migration: Bump mapping apiVersion from 0.0.4 to 0.0.5
  Skip migration: Bump mapping specVersion from 0.0.1 to 0.0.2
✔ Apply migrations
✔ Load subgraph from subgraph.yaml
✔ Generate types for data source templates
✔ Load GraphQL schema from schema.graphql
  Write types to generated/schema.ts
✔ Generate types for GraphQL schema

Types generated successfully

Done in 1.60s.
~~~
