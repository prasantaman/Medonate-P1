// Working
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Address};


#[derive(Clone, Debug)]
#[contracttype]
pub struct Account {
    address: Address,
    amount: u32,
}


#[contract]
pub struct Medonate;


#[contractimpl]
impl Medonate {
    pub fn donate_amount(env: Env, address: Address, amount: u32) {
        if let Some(mut party) = env.storage().persistent().get::<_, Account>(&address) {
            party.amount += amount;
            env.storage().persistent().set(&address, &party);
        } else {
            panic!("Not found");
        }
    }

    
    pub fn checkbalance(env: Env, address: Address) -> Option<u32> {
        if let Some(party) = env.storage().persistent().get::<_, Account>(&address) {
            Some(party.amount)
        } else {
            panic!("Account Not Found!");
        }
    }
    

    pub fn add_new_account(env: Env, address: Address, amount: u32) {
        let party = Account {
            address: address.clone(),
            amount,
        };
        env.storage().persistent().set(&address, &party);
    }
}