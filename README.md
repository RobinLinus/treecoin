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

### Scalability Goals
- PayPal: [200 Million Accounts](https://www.statista.com/statistics/218493/paypals-total-active-registered-accounts-from-2010/) and [240  transactions/second](https://www.statista.com/statistics/419778/paypal-annual-payments)
- Bitcoin: [1 Million Addresses](https://blockchain.info/charts/n-unique-addresses) and [7 transactions/second](https://blockchain.info/charts/n-transactions?timespan=all)

We need to increase Bitcoin's current capacity by a factor of about 40 to scale to PayPal's throughput.


## Architecture Overview
The architecture is based on Bitcoin's design and introduces the following extensions:
- [_State Tree_: an efficient representation of the full chain state.](docs/state-tree.md)
  -  _Headers tree_: an index for blocks in the chain.
    - _Proof of proofs of work_: compact proof of longest chain.
  - TXO-IDs: an index for all transaction outputs.
  - Address index: an index for the unspent outputs of addresses.
- [Compact Transactions](docs/transactions.md): compact UTXOs and signatures.
- [Network](docs/network.md)
  - Backbone Network: miners and full nodes.
  - Browser bridges: seeding for nano nodes and block relay into nano network.
  - Nano Network: self-serving nano nodes.

## Implementation
- Rust
  - [Rust Bitcoin Library](https://github.com/rust-bitcoin/rust-bitcoin)
- WebAssembly
