# lovicash protocol architecture

## design goals 
- censorship-resistant payment network
- minimum 100 tx/s ( to compete with paypal )
- nano clients
  - censorship-resistant
  - quick sync
  - web-based for simple usability

## Network Overview
1. Mainnet ( Miners and high-stake users )
2. Browser Bridge 
3. Browser Network

### Backbone Network
Purpose:
- censorship resistant consensus
- simple and resilient blockchain architecture
- high tx throughput

### Browser Bridges
Required for browsers to communicate with the backbone.

Purpose:
- Initial WebRTC Signaling
- 

Requirements:
- Open Port
- Domain
- SSL Certificate

Research Idea: Installation-free, browser-based websocket-to-webrtc bridges via insecure origins/data urls?

### Browser Network
Purpose:
- distributed storage of the state
- distributed updates of the state by applying new blocks from the bridge
- serve all nano clients' queries within the browser network

Requirements:
- P2P Network
- WebRTC Signaling
- Distributed Hash Table

## Censensus Mechanism
For simplicity we use double sha256 proof of work.

## Nano State
For simplicity and performance we're looking for a more efficient state represantation than UTXO set or Merkle PATRICIA trees.

### Headers chain
The headers chain is the root of trust for nano clients. Properties: 
- as concise as possible
  - in bitcoin: 80bytes/header 3.4 MB/year
  - scales linearily with block frequency.
    - 1 min blocktime: => 34Mb/year
  - simplified PoPoW: "natural checkpoints" a chain becomes more trustworthy if it contains "heavy" outliers. 
- header size 
  - 32 bytes prevHash
  - 32 bytes txHash
  - 8 byte nonce 
  - 4 byte timestamp
  - 4 byte version
  
In our case we're only interested in proving an address' balance. We reuse the existing block structure to create an authenticated data set. The UTXO set is always flattened to addresses, though the full state is not represented in one piece but "block by block":
- transactions are in a merkle tree
  - there are two indices on top of the merkle tree: sender index and receiver index
    - both indexes are _static_, balanced binary search trees
    - their structure is merkled seperately within the merkle tree
    - they provide bitmasks to efficiently find transactions within the merkle tree [ log(TXs) ]
- write the resulting balance into the block such that every addresses' state gets flattened in every block it occurs in.
