# Lovicash: A P2P Payment System

## Design Goals
- Trust-less, censorship-resistant payment network
- Scalability
  - Up to 25 tx/s ( 25% of Paypal's transaction volume )
  - Up to 1 000 000 concurrent clients
- Nano Nodes
  - Trust-less _nano sync_ ( "sync by downloading less than a song" ~ 3 MB )
  - Web-based for simple usability ( "click link to start syncing" )

## Architecture Overview
The architecture is based on bitcoin's design and introduces the following extensions:
- [_State Tree_: an efficient and authenticated representation of the global state.](docs/state-tree.md)
  -  _Headers tree_: an index for blocks in the chain.
    - _Proof of proofs of work_: compact proof of longest chain.
  - TXO-IDs: an index for all transaction outputs.
  - UTXO bit vector: an index for all unspent outputs.
  - Address index: an index for the unspent outputs of _addresses_.
  - Transactions: compact UTXOs and signatures.
- [Network](docs/network.md)
  - Backbone Network: miners and full nodes.
  - Browser bridges: seeding for nano nodes and block relay into nano network.
  - Nano Network: self-serving nano nodes.

## Implementation
- Rust
- WebAssembly
