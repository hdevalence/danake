# Implementation

Danake is implemented in Rust. The proof statements are written
declaratively, using the [`zkp`](https://docs.rs/zkp) crate to automatically
derive implementations of each proof statement. Rangeproofs are provided by
the [`bulletproofs`](https://docs.rs/bulletproofs) library, and they are
composed with the other proof statements using [Merlin
transcripts](https://merlin.cool).

# Implementation status

## Core functionality

- [ ] Wallet functionality
  - [x] keygen
  - [x] issuance
  - [ ] rollover
  - [x] topup
- [ ] Token functionality
  - [ ] keygen
  - [ ] purchase
  - [ ] rollover
  - [ ] spend
- [ ] Proper transcript design
- [ ] Nullifier queries (double-spend prevention)
- [ ] Epoch-aware keygen (should be able to generate keys for every epoch from a single root key)
- [ ] Simulator
  - [ ] `Arbitrary` impl generating a stream of protocol events
  - [ ] proptest checker for event streams

## Integrations

These would make Danake more practically useful:

- [ ] `danake-proxy`: reverse proxy that sits in front of an HTTP service and meters an HTTP API.