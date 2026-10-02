#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    symbol_short,
    vec,
    Env,
    Symbol,
    Vec,
};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, nombre: Symbol) -> Vec<Symbol> {
        vec![
            &env,
            symbol_short!("Hola"),
            nombre,
        ]
    }
}

