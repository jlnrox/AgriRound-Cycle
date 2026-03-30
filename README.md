PROJECT NAME: AgriRound Cycle

PROBLEM: Smallholder farmers in rural areas of the Philippines face difficulty managing their group savings cycles because contributions are often late, records are tracked manually, and communication gaps lead to delayed or disputed fund releases.
SOLUTION: AgriRound Cycle uses a Soroban smart contract to enforce cooperative savings cycles, where farmers submit weekly USDC contributions on-chain and the contract automatically releases the pooled funds to the scheduled member only when all required payments are completed, removing the need for manual coordination or trust.

STELLAR FEATURES USED
Soroban smart contract (cycle logic + auto payout)
USDC transfers (weekly contributions + cycle payout)
Built-in DEX (optional USDC↔XLM conversion for flexibility)
Custom tokens (optional “Cycle Credits” to track each round)
Trustlines (for holding USDC or Cycle Credits) 

TARGET USERS:  Rice, corn, vegetable, and fruit farmers in rural cooperatives across the Philippines participating in rotating savings groups; cooperative leaders managing pooled funds; and agri-suppliers relying on timely payments for seeds, fertilizers, and other seasonal inputs.

CORE FEATURE (MVP): A farmer joins a savings cycle by interacting with a Soroban contract and submitting a weekly USDC contribution; the contract records each payment on-chain, verifies when all members have contributed for the round, and automatically triggers the payout to the assigned farmer’s wallet without requiring manual approval.

THEME: Savings & Lending → Rotational Group Savings + Farmer Payment Coordination (farmers contribute weekly USDC to a shared cycle and receive instant Stellar payouts when it’s their scheduled turn)


Contract Explorer Link:
https://stellar.expert/explorer/testnet/contract/CCRZHSAI6XMP42PF45KF7PDQ4YHY5RF3S2COKRQACK545FSSHCRPWK4A
