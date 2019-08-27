# research

## <a href="https://github.com/WebOfTrustInfo/rwot9-prague/blob/master/topics-and-advance-readings/Decentralizing-Reputation-with-DID.md">RWOT9 Decentralizing Reputation with DID</a>

*The transaction cost has traditionally been reduced by introducing a centralized intermediary as technology operator. This avoids lagging open standards and reduces the cost for the peers to operate self-sovereign technology to execute the transaction. However, as technology cost drops with Moore’s law and standards evolve, the transaction case for centralization becomes moot, leaving search and risk as practical barriers to decentralization. Search for a suitable peer and risk of transacting with that peer are related since a good search experience would consider risk as a major influence in the rankings.*

Reputation is highly **contextual**...**A decentralized reputation solution must provide context, a negligible increase in transaction costs, and high resistance to gaming by either the peers to a transaction or their competitors.**

### impl details

> Scoping context in a decentralized way is a governance issue => One natural way to scope reputation linked to SSI would be at the level of the search engine or directory. The governance entity that brings anonymous peers together based on their attributes and credentials might also supply the formula for calculating reputation when a transaction actually results from the match. The directory governance entity could also be held responsible for securing and posting the reputation results, possibly as a verifiable credential. ~ [rwot9-prague/topics-and-advance-readings/Decentralizing-Reputation-with-DID](https://github.com/WebOfTrustInfo/rwot9-prague/blob/master/topics-and-advance-readings/Decentralizing-Reputation-with-DID.md)

Rather than using a single parachain for identity, each parachain will likely incorporate some notion of identity to manage reputation in the context of the chain-specific economy.  

#### References

* https://bloom.co/whitepaper.pdf
* https://nvlpubs.nist.gov/nistpubs/CSWP/NIST.CSWP.07092019-draft.pdf
* https://w3c-ccg.github.io/did-spec/
	1. read authors writings (especially Christopher Allen and Drummond Reed)
	2. reach out to authors once we have an implementation

**instead of searching for generic solutions, choose specific scenarios that are sufficiently general and design for those scenarios** => this hedges the possibility of creating another useless standard that defies adoption (likewise, target scenarios that require robust reputation based on internal identity...which is a lot of things...)
