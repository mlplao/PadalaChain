https://stellar.expert/explorer/testnet/tx/a5adc223040a77c4b3724544fb7b14ae407ff34ac266a2ed44a0abd6fc800afa
<img width="1527" height="665" alt="stellarshit" src="https://github.com/user-attachments/assets/2cd92962-4395-43fc-af31-f78f0a6bdfe2" />
Contract ID: CAOLVL3L3Z6KXKQU6UIXBC6R7MSDFTVAOTDCW7TSJN5LT64E7AQRTCBM
# PadalaChain
Instant, near-zero-fee remittances for Filipino overseas workers via Stellar.

## Problem and Solution
* **Problem:** Filipino domestic workers abroad lose 7–10% of each remittance to fees and wait days for transfers to reach family members in the Philippines.
* **Solution:** A mobile-first app that enables USDC transfers on Stellar to registered cash-out agents, who release PHP to recipients instantly. Fees under $0.01, settlement in 5 seconds.

## Timeline
| Phase | Duration | Deliverable |
| :--- | :--- | :--- |
| **Smart contract** | Day 1–2 | Agent registry + transfer logic |
| **Frontend** | Day 3–4 | Mobile web app with wallet connect |
| **Integration** | Day 5 | Testnet deploy + demo |

## Stellar Features Used
* **USDC transfers:** Stable value for international transfers.
* **Trustlines:** Agent verification and asset acceptance.
* **Built-in DEX:** PHP liquidity for local conversion.

## Vision and Purpose
Reduce the $30B+ annual OFW remittance market's friction, returning hundreds of dollars per year to families who need it most.

## Prerequisites
* Rust 1.74+
* Soroban CLI 21.0.0+
* Node.js 18+ (for frontend)

## How to Build
To compile the smart contract, run the following command in your terminal:
```bash
soroban contract build
