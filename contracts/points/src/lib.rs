#![no_std]
use soroban_sdk::{contract, contractimpl,contracttype,Symbol,symbol_short, Address, Env};

const ADMIN: Symbol = symbol_short!("admin");

#[contracttype]
pub enum DataType {
    Points(Address)
}

#[contract]
pub struct LoyaltyPointsContract;

#[contractimpl]
impl LoyaltyPointsContract {
    /// Construct the contract with a provided administrator.
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&ADMIN, &admin);
    }

    pub fn set_points(env:Env, user:Address,amount:u64){
        let key = DataType::Points(user); 
        env.storage().persistent().set(&key, &amount);
    }
    pub fn get_points(env:Env,user:Address)->u64{
        let key = DataType::Points(user); 
        let amount:u64 = env.storage().persistent().get(&key).unwrap_or(0);
        amount
    }
    pub fn redeem_points(env:Env, user:Address, points:u64)->bool{
       user.require_auth(); 
       let key = DataType::Points(user);
       let current_points = env.storage().persistent().get(&key).unwrap_or(0);

       if current_points >= points {
          let new_points = current_points - points;
          env.storage().persistent().set(&key, &new_points);
          true
       } else {
          false
       }
    }
}

mod test;
