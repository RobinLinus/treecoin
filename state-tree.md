# State Tree

## Design Goals
- fast, authenticated queries on the blockchain 
- prunable blockchain data
- fast append
- Simple

## Overview
The State Tree is a modified blockchain: an authenticated, indexed, append-only _binary tree_. 
Classic blockcains organize a block's transactions in a Merkle tree. We apply this approch to the blocks to build a Merkle tree on top of the whole blockchain. 

![alt text](datamodel.png "State Tree")

## Indexes 

### Block Ids 
A block's id is simply its block height. This fits well with the binary tree because the binary representation of the blockId corresponds to its path in the state tree.
This way it is simple to answer a query for a block inclusion proof.

### Transaction Ids
Since transactions are merkled within a block, we can address them canonically by their path in the binary TX tree.
 
### TX Output Id
The id of an output of an transaction consists of:
- block index
- transaction index
- output index

### Properties
- Data identifiers are 64 bit numbers:
  - block index: 39 bit ( enough for 12 billion blocks )
  - tx index: 17 bit ( enough for 100 000 tx / block )
  - output index: 8 bit ( enough for 256 outputs )
- not random! blocks, transactions and outputs are indexed incrementally. Index == precise point in time
