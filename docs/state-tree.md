# State Tree

## Design Goals
- Efficient balance proofs for every address in the blockchain
- Fast state updates
- Prunable chain state
- Simple design

## Related Work
- [Ultimate blockchain compression](https://bitcointalk.org/index.php?topic=88208.0) - the general idea that the blockchain can be compressed to achieve "trust-free lite nodes".
- [Merkle-PATRICIA Trie](https://github.com/ethereum/wiki/wiki/Patricia-Tree) as used in systems like Ethereum. They introduce substantial computational and storage overhead, and they would be hard to update for the nano network because knowledge of almost the full tree is needed to compute the state transition defined by a random block. We introduce a more efficient representation.

## Overview
The _State Tree_ is a modified blockchain: an authenticated, indexed, append-only _binary tree_.

-  _Headers tree_: an index for all blocks in the chain.
  - _Proof of proofs of work_: a compact _proof of longest chain_.
- TXO-IDs: an index for all transaction outputs.
- Address index: an index for the unspent outputs of an _address_.

## Headers Tree

Classic blockcains organize a block's transactions in a Merkle tree. We apply this approch also to the blocks to build a Merkle tree on top of the full blockchain.
This struture is known as [Merkle Mountain Range](https://github.com/opentimestamps/opentimestamps-server/blob/master/doc/merkle-mountain-range.md) as [discussed in bitcoin](https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2016-June/012758.html).
The headers tree and the transaction trees in the blocks form the _state tree_:

![alt text](images/state-tree.png "State Tree")

Note that full nodes require only the most recent path to insert a block into the state tree. This results in an overhead of only O(log(N)). Assuming the current state of Bitcoin that is less than `log2(600000) * 32 bytes ~ 620 bytes`!

The total overhead for the nano network to host the full headers tree is O(N) or currently about `600000 * 32 bytes = 19.2 MB`.

### Proofs of Proof of Work
Given another binary search tree on the most "heavy" blocks ( PoW >> difficulty target ), we can create the same effect as [Proofs of Proof of Work](https://eprint.iacr.org/2017/963.pdf) or [Efficient SPV proofs](https://www.blockstream.com/sidechains.pdf) and therefore compress the headers chain proof logarithmically. This is very important to reduce the block time without increasing the sync load on nano clients.
This tree is simply represented and merkled within the nodes of the state tree. The additional overhead is about one byte per node.

![alt text](images/popow.png "State Tree")

Assuming Bitcoin's current state at a block height of 600000, nano nodes would download a chain proof of about:
- `log2(600000)*80 + log2(600000)*log2(600000)*32 ~ 14Kb`
- or `log2(6000000)*80 + log2(6000000)*log2(6000000)*32 ~ 18Kb` assuming a 10x faster block time of 1 minute.

#### Probabilistic Proof of Proofs of Work
To further increase confidence in the chain's consistency nano nodes can also query for _random_ headers by header id. This sampling exponentially increases the probabilistically proved work (as long as _every_ random query is answered). This is also beneficial to the load balancing in the nano network because it incentivizes nano nodes to replicate the full headers chain redundantly.

This concept is very similar to [FlyClient - super light client for cryptocurrencies](https://scalingbitcoin.org/stanford2017/Day1/flyclientscalingbitcoin.pptx.pdf).

## State Tree Indexes

### Block Index
A block's id is simply its block height. This fits well with the headers tree because the binary representation of the blockId corresponds to its path in the headers tree.
This way it is simple to answer a query for a block inclusion proof.

### Transaction Index
Since transactions are merkled within a block, we can address them canonically by their path in the transactions tree.

### Transaction Output Index
The id of an output of a transaction consists of:
- block index
- transaction index
- output index

### Properties
- Bit representation of an identifier corresponds to a path in the state tree and therefore to a unique inclusion proof (of logarithmic size).
- Identifiers are 64 bit numbers:
  - block index: 39 bit ( enough for 12 billion blocks )
  - tx index: 17 bit ( enough for 100 000 tx / block )
  - output index: 8 bit ( enough for 256 outputs / tx )
- not random! blocks, transactions and outputs are indexed incrementally. Index == precise point in time

## Address Index
The Address Index is a simple structure to query the  balance proof for an address. Transaction outputs contain [the balance of an address](transactions.md#address-balance) and therefore the UTXO set grows like `O("number of addresses")` instead of `O("number of transactions")`. Thus we simply sort the set of all UTXO ids by lexicographic order of the corresponding address to get a simple map from address to UTXO id.

Assuming 1 Million addresses and 40 bit UTXO ids, we get an Address Index size of 5 MB. To further scale this approach we can introduce another Merkle tree to chunk the Address Index into chunks of about 500 KB. The required overhead for this _chunks tree_ is `O( "number of addresses" / 100000 )` and thus neglectable.

This design allows nano nodes to perform a binary search to look up an address.
  - The overhead is about `O( log(N) * log(M) )` whereas `N` is the UTXO count and `M` is the TXO count. In Bitcoin's current state that's about 20 KB per address balance proof.
  - Blocks contain a commitment to the chunks tree root, thus the Address Index allows _address absence proofs_, too.

This is neither the most optimal representation nor query. Though this inefficiency is practical, the design is simple and it even has some advantages:
- Enhanced privacy
  - Downloading the chunks tree is privacy preserving. Furthermore users can split their queries among different peers such that it becomes harder for an attacker to unveil their addresses.
- Enhanced load balancing
  - The Nano network should scale infinitely. Thus it needs to enforce nano nodes to download more than what's relevant only to themselves. Therefore _not_ optimizing queries perfectly enforces nano clients to help scale the redundancy of state storage.

Possible Optimizations:
- Users using multiple addresses can easily "mine" their addresses such that they probably end up in the same chunks, to reduce the amount of chunks needed to prove all their addresses' balances.
- Addresses are uniformly distributed and therefore it is easy to calculate an educated guess for the chunk in which an address lies most likely. That reduces the query's overhead since a user knows in advance which chunk he needs.
- We can achieve the same effect by providing a prefix for the first and last entry in every chunk. These prefixes are propagated up the chunks tree to create a binary search tree.
- The Address Index does not contain random values, but distinct patterns that are highly compressable:
  - The TXO ids are almost perfectly incremental. the natural order on TXO ids is represents points in time.
  - As long as cash is flowing, there are more recent UTXOs than old ones.
    - This can be exploited further by introducing a simple time order into the chunks.


# Optimizations

### Delayed Commitments
We do not want to introduce new consensus critical computation into the design of bitcoin. Therefore all indices are [delayed commitments](https://petertodd.org/2016/delayed-txo-commitments): A block does not contain a commitment to the most recent state, but to the state of a predetermined predecessor (i.e. from 6 blocks ago) such that miners can precompute the computationally intensive result of all state transitions before a new block is mined. This way we do not increase the time to verify or mine a new block.

### Fast Merkle Trees
We make heavy use of Merkle trees thus their performance is consensus critical. [Fast Merkle Trees](https://gist.github.com/maaku/41b0054de0731321d23e9da90ba4ee0a) are a more efficient alternative to the current Merkle tree implementation in bitcoin.
