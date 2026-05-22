# Title
TrustEscrow - A Decentralized Milestone-Based Escrow Protocol

# Description
In the freelance ecosystem, establishing mutual trust remains a critical friction point. Student developers frequently face the risk of non-payment upon project delivery, while clients are equally hesitant to provide upfront capital due to concerns over project abandonment or substandard code quality.

TrustEscrow mitigates this vulnerability by acting as an incorruptible, decentralized intermediary (Smart Escrow) deployed on the Soroban (Stellar) network. The protocol cryptographically locks project funds within a smart contract and strictly automates the disbursement of fractional payments (milestones) only upon client verification of progress. Enforcing the "Code is Law" paradigm, this architecture guarantees equity, absolute transparency, and financial security for all participating entities.

# Features
* **Cryptographic Fund Locking:** Clients initialize projects by defining temporal constraints and depositing funds (Native XLM or USDC) into the smart contract. These assets are immutably locked and cannot be arbitrarily withdrawn.
* **Automated Milestone Disbursement:** Upon the successful completion of a designated project phase, client authorization triggers the smart contract to autonomously execute a secure fund transfer to the freelancer's wallet.
* **Algorithmic Fee Extraction:** Integrating a sustainable business model, the protocol automatically deducts a predefined platform fee (e.g., 200 basis points or 2%) during the disbursement process, routing it to the administrative reserve.
* **Time-Locked Refund Mechanism:** To prevent capital stagnation, the contract incorporates a timestamp-based deadline. If the project remains unresolved past this temporal threshold, clients are granted the authorization to execute a refund protocol.
* **Decentralized Arbitration Protocol:** In the event of a dispute, the project state transitions to `Disputed`. An authorized arbitrator (Admin) assesses the empirical evidence to execute a fair, deterministic reallocation of the remaining locked funds.

# Contract
**Contract Link:** [https://stellar.expert/explorer/testnet/contract/CDGSAGSPAFROF6JX3Q474J3UDE4CNDBRBKHN6NU3ONYM2IASFRSZX4WD]

**Contract's screenshot:**



# Future Scopes
* **Dynamic Array Implementation:** Upgrading the smart contract state to support dynamic arrays for granular tracking of multi-stage milestones with variable disbursement ratios.
* **Robust Backend Architecture:** Architecting a comprehensive off-chain backend via RPC to handle user authentication, real-time event indexing, and historical transaction management.
* **On-Chain Reputation System:** Integrating an immutable rating algorithm that leverages blockchain data to construct verifiable, tamper-proof professional profiles for freelancers.
* **Custom Asset Integration:** Expanding the smart contract's token compatibility to support localized digital assets, such as specific campus tokens within educational ecosystems.

# Profile
* **Name:** Phan Van Truong Huy 
* **Orientation:** First-year undergraduate student.
.