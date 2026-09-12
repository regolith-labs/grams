# Grams

Users can send ORE commmunity memecoins to this address to permanently lock the supply:

```
GHRBYPA4cujFwfyhNNm6NLTh4egdTrcz7xkBbEwM4xX
```

Any ORE or stORE received by the contract (e.g. via rebates earned on Stonksfun, OTC, or Pumpfun platforms) will be automatically buried.

Anyone can call `unwrap` + `bury` to convert stORE into ORE and burn all ORE in the vault.

## Program

- [`Bury`](program/src/bury.rs) – Burns all ORE in the receivables ATA.
- [`Unwrap`](program/src/unwrap.rs) – Unwraps all stORE in the receivables ATA into ORE via CPI to ore-lst.

## API

- [`Consts`](api/src/consts.rs) – Program constants.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.
- [`SDK`](api/src/sdk.rs) – Instruction builders.
- [`State`](api/src/state/mod.rs) – PDA definitions.

## CLI

```
# Print receivables address and token balances
RPC=<rpc_url> grams-cli balance

# Submit an unwrap + bury transaction
RPC=<rpc_url> KEYPAIR=<path_to_keypair> grams-cli run
```

## Build

```
cargo build-sbf
```
