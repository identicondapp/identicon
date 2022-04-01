export PARENT=identicon.testnet
export CONTRACT=contract0.$PARENT

near delete $CONTRACT $PARENT

near create-account $CONTRACT --initialBalance 5 --masterAccount $PARENT

near deploy $CONTRACT --wasmFile res/identicon.wasm
