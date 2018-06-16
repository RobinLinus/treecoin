# Blocks

## Headers
The header defines the most controversial parameters of a blockchain: consensus mechanism and maximum capacity.


## Consensus Mechanism
### Proof of Work

- The [environmental costs of bitcoin](https://j3l7h.de/talks/2012-09-20_Environmental_Cost_of_Bitcoin.pdf) are significant.
- Proof of work is rooted in thermodynamics limits and therefore [hard to optimize in terms of "waste"](https://download.wpsoftware.net/bitcoin/asic-faq.pdf) of energy and hardware, simply because less energy equals lower security.
- Proof of Work incentivizes miners to use dedicated mining hardware (ASICs = "Application Specific Integrated Circut"), because mining is a null-sum game and intentionally provokes an arms race for the best mining hardware to overpower any possible attacker.
- ASIC mining trivially leads to centralization of ASIC production just because chip production is highly centralized in China.
- For ASIC manufacturers it is cheap to design an ASIC for every PoW. Up to today every PoW which claimed to be "ASIC-resitant" failed as soon as its currency became valuable and therefore ASICs became profitable.
- ASICs are probably unavoidable.
- With ASICs, mining centralization in China is unavoidable.
- There are scientist running experiments to design algorithms which are ASIC-resistant such as [Cuckoo Cycles](https://github.com/tromp/cuckoo).

### Proof of Stake
- [Proof of Stake](https://download.wpsoftware.net/bitcoin/pos.pdf)
  - [Ouborous](https://eprint.iacr.org/2016/889.pdf)
  - [Stake-Bleeding Attacks on Proof-of-Stake Blockchains](https://eprint.iacr.org/2018/248.pdf)

### Alternatives
- [Hybrid Algorithms]
- [Proof of Space](https://en.wikipedia.org/wiki/Proof-of-space) is a means of showing that one has a legitimate interest in a service by allocating a non-trivial amount of memory or disk space to solve a challenge presented by the service provider.
- [Proof of Authority]()
- [Proof of Trust]()

## Maximum Capacity

#### Block Size

#### Block Time
- [Block Time vs Orphan Rates vs Security](https://eprint.iacr.org/2016/555.pdf)
Bitcoin has a conservative blocktime of ten minutes.



- [Sidechains](https://www.blockstream.com/sidechains.pdf)
- [Block size following technological growth](https://github.com/bitcoin/bips/blob/master/bip-0103.mediawiki)
