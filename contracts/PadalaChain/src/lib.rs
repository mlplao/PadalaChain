#![no_std]
#![allow(deprecated)] // Hides the minor event publishing warning for now

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String, token};

/// Storage keys for the contract
#[contracttype]
pub enum DataKey {
    Agents,           // Map of registered cash-out agents
    Transfers,        // Transfer records
    TransferCount,    // Counter for transfer IDs
}

/// Transfer status enum
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransferStatus {
    Pending,
    Completed,
    Cancelled,
}

/// Transfer record structure
#[contracttype]
#[derive(Clone, Debug)]
pub struct Transfer {
    pub id: u64,
    pub sender: Address,
    pub agent: Address,
    pub amount: i128,
    pub recipient_phone: String,  // Changed to String to support standard phone numbers
    pub status: TransferStatus,
}

#[contract]
pub struct PadalaChain;

#[contractimpl]
impl PadalaChain {
    /// Registers an address as a verified cash-out agent
    pub fn register_agent(env: Env, agent: Address) {
        agent.require_auth();
        
        let mut agents: Map<Address, bool> = env
            .storage()
            .persistent()
            .get(&DataKey::Agents)
            .unwrap_or(Map::new(&env));
            
        agents.set(agent.clone(), true);
        env.storage().persistent().set(&DataKey::Agents, &agents);
        
        // Extend TTL to keep data alive on the network
        env.storage().persistent().extend_ttl(&DataKey::Agents, 100_000, 500_000);
    }

    /// Creates a new remittance transfer and locks funds
    pub fn create_transfer(
        env: Env,
        sender: Address,
        agent: Address,
        token: Address,          // USDC Token Address
        amount: i128,
        recipient_phone: String, 
    ) -> u64 {
        sender.require_auth();

        // 1. Verify agent is registered
        let agents: Map<Address, bool> = env
            .storage()
            .persistent()
            .get(&DataKey::Agents)
            .unwrap_or(Map::new(&env));
            
        if !agents.get(agent.clone()).unwrap_or(false) {
            panic!("Agent not registered");
        }

        // 2. Transfer the tokens! (Sender locks funds to Agent)
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&sender, &agent, &amount);

        // 3. Get and increment transfer counter
        let mut count: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::TransferCount)
            .unwrap_or(0);
        count += 1;

        // 4. Create transfer record
        let transfer = Transfer {
            id: count,
            sender: sender.clone(),
            agent: agent.clone(),
            amount,
            recipient_phone,
            status: TransferStatus::Pending,
        };

        // 5. Store transfer
        let mut transfers: Map<u64, Transfer> = env
            .storage()
            .persistent()
            .get(&DataKey::Transfers)
            .unwrap_or(Map::new(&env));
            
        transfers.set(count, transfer);
        env.storage().persistent().set(&DataKey::Transfers, &transfers);
        env.storage().persistent().set(&DataKey::TransferCount, &count);
        
        // Extend TTLs
        env.storage().persistent().extend_ttl(&DataKey::Transfers, 100_000, 500_000);
        env.storage().persistent().extend_ttl(&DataKey::TransferCount, 100_000, 500_000);

        // Emit event for agent notification
        env.events().publish(
            (symbol_short!("transfer"), sender),
            (count, agent, amount),
        );
        
        count
    }

    /// Agent marks transfer as completed after releasing cash
    pub fn complete_transfer(env: Env, agent: Address, transfer_id: u64) {
        agent.require_auth();
        
        let mut transfers: Map<u64, Transfer> = env
            .storage()
            .persistent()
            .get(&DataKey::Transfers)
            .unwrap_or(Map::new(&env));
            
        let mut transfer = transfers.get(transfer_id).expect("Transfer not found");
        
        if transfer.agent != agent {
            panic!("Unauthorized agent");
        }
        if transfer.status != TransferStatus::Pending {
            panic!("Transfer not pending");
        }
        
        transfer.status = TransferStatus::Completed;
        transfers.set(transfer_id, transfer);
        env.storage().persistent().set(&DataKey::Transfers, &transfers);
        
        // Extend TTL
        env.storage().persistent().extend_ttl(&DataKey::Transfers, 100_000, 500_000);
        
        env.events().publish(
            (symbol_short!("complete"), agent),
            transfer_id,
        );
    }

    /// Query transfer details
    pub fn get_transfer(env: Env, transfer_id: u64) -> Transfer {
        let transfers: Map<u64, Transfer> = env
            .storage()
            .persistent()
            .get(&DataKey::Transfers)
            .unwrap_or(Map::new(&env));
            
        transfers.get(transfer_id).expect("Transfer not found")
    }

    /// Check if an address is a registered agent
    pub fn is_agent(env: Env, addr: Address) -> bool {
        let agents: Map<Address, bool> = env
            .storage()
            .persistent()
            .get(&DataKey::Agents)
            .unwrap_or(Map::new(&env));
            
        agents.get(addr).unwrap_or(false)
    }
}