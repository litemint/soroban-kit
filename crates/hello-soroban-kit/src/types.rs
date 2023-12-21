/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use soroban_kit::{key_constraint, soroban_tools, storage};
use soroban_sdk::{contracttype, Address, Bytes, Env, Symbol};

// Optional but recommended.
// Use `key_constraint` to apply a constraint to the Key
// that will be used to operate the storage.
#[contracttype]
#[key_constraint(DataKeyConstraint)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Newcomer,
}

// Use `storage` to implement the desired storage for your
// custom contract type. We also apply the DataKeyConstraint.
#[contracttype]
#[storage(Instance, DataKeyConstraint)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Data {
    pub newcomer: Symbol,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    Opened,
    Running(i32),
    Closed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Region {
    Global,
    Specific(i32),
    Circuit,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Player {
    Alice,
    Bob,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Phase {
    Start,
    Committing(Player),
    Revealing(Player),
    Completed(Player),
    End,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Domain {
    Players(Player),
    Game,
}

#[key_constraint(MessageKeyConstraint)]
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageKey {
    Topic(Bytes),
    AuthorizedBroker,
}

#[storage(Instance, MessageKeyConstraint)]
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub data: Bytes,
    pub timestamp: u64,
}

#[key_constraint(WhitelistKeyConstraint)]
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WhitelistKey {
    Broker,
}

#[storage(Instance, WhitelistKeyConstraint)]
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Whitelist {
    pub broker: Address,
}
