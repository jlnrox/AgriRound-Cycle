#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Env, String, Map, Symbol, symbol_short,
};

#[contract]
pub struct AgriRoundCycle;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Farmers,
    Contributions,
}

#[contractimpl]
impl AgriRoundCycle {

    pub fn init_cycle(env: Env, cycle_id: String) -> Symbol {
        let farmers: Map<String, bool> = Map::new(&env);
        let contributions: Map<String, u32> = Map::new(&env);

        env.storage().instance().set(&DataKey::Farmers, &farmers);
        env.storage().instance().set(&DataKey::Contributions, &contributions);

        env.storage().instance().set(&cycle_id, &0u32);

        symbol_short!("OK")
    }

    pub fn join_cycle(env: Env, farmer: String) {
        let mut farmers: Map<String, bool> = env
            .storage()
            .instance()
            .get(&DataKey::Farmers)
            .unwrap_or(Map::new(&env));

        farmers.set(farmer, true);

        env.storage().instance().set(&DataKey::Farmers, &farmers);
    }

    pub fn contribute(env: Env, farmer: String, amount: u32) {
        let mut contributions: Map<String, u32> = env
            .storage()
            .instance()
            .get(&DataKey::Contributions)
            .unwrap_or(Map::new(&env));

        let current = contributions.get(farmer.clone()).unwrap_or(0);
        contributions.set(farmer, current + amount);

        env.storage()
            .instance()
            .set(&DataKey::Contributions, &contributions);
    }

    pub fn check_cycle_status(env: Env) -> Symbol {
        let farmers: Map<String, bool> = env
            .storage()
            .instance()
            .get(&DataKey::Farmers)
            .unwrap_or(Map::new(&env));

        let contributions: Map<String, u32> = env
            .storage()
            .instance()
            .get(&DataKey::Contributions)
            .unwrap_or(Map::new(&env));

        let mut all_paid = true;

        for (farmer, _) in farmers.iter() {
            if contributions.get(farmer).unwrap_or(0) == 0 {
                all_paid = false;
            }
        }

        if all_paid {
            symbol_short!("READY")
        } else {
            symbol_short!("PENDING")
        }
    }

    pub fn release_payout(_env: Env, _farmer: String) -> Symbol {
  
        symbol_short!("PAID")
    }

    pub fn hello(_env: Env) -> u32 {
        1
    }
}