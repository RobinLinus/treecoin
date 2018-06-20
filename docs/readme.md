# Treecoin: A P2P Payment System

## Design Goals
A scalable, trustless payment network designed for [mainstream adoption](https://en.wikipedia.org/wiki/Technology_adoption_lifecycle).
- Simplicity
  - ... for Security.
  - ... for Scalability.
  - ... for Usability.
- Scalability
  - Up to 100 tx/s ( Paypal's transaction volume )
  - Up to 1 000 000 concurrent nodes.
- Usability
  - Trustless _nano nodes_ ( "sync by downloading less than a song" ~ 3 MB )
  - Web-based for most simple on-boarding ( "click to sync" )
  - Beautiful user interfaces and user experiences.


## Architecture Overview
The architecture is based on Bitcoin's design and introduces the following extensions:
- [_State Tree_: an efficient representation of the full chain state.](state-tree.md)
  -  _Headers tree_: an index for blocks in the chain.
    - _Proof of proofs of work_: compact proof of longest chain.
  - TXO-IDs: an index for all transaction outputs.
  - Address index: an index for the unspent outputs of addresses.
- [Compact Transactions](transactions.md): compact UTXOs and signatures.
- [Network](network.md)
  - Backbone Network: miners and full nodes.
  - Browser bridges: seeding for nano nodes and block relay into nano network.
  - Nano Network: self-serving nano nodes.

## Implementation
- Rust
  - [Rust Bitcoin Library](https://github.com/rust-bitcoin/rust-bitcoin)
- WebAssembly
