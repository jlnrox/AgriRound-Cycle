#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, symbol_short};

#[test]
fn test_agri_round_cycle_basic_flow() {
    let env = Env::default();
    let contract_id = env.register(AgriRoundCycle, ());

    let client = AgriRoundCycleClient::new(&env, &contract_id);

   
    let result = client.init_cycle(
        &String::from_str(&env, "CYCLE-1"),
        &1,
    );
    assert_eq!(result, symbol_short!("OK"));

    let farmer1 = String::from_str(&env, "FARMER_A");
    let farmer2 = String::from_str(&env, "FARMER_B");

    client.join_cycle(&farmer1);
    client.join_cycle(&farmer2);

  
    client.contribute(&farmer1, &100);
    client.contribute(&farmer2, &100);

   
    let status = client.check_cycle_status();
    assert_eq!(status, symbol_short!("READY"));

   
    let payout_result = client.release_payout(&farmer1);

    assert_eq!(payout_result, symbol_short!("PAID"));
}