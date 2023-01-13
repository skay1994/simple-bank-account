use crate::structs::User;

use rusty_money::{Money, iso};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Account {
    pub user: User,
    balance: i32,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

impl Account {
    pub fn new(user: User, initial_balance: i32) -> Account {
        Account {
            user,
            balance: initial_balance,
            created_at: Utc::now(),
            update_at: Utc::now()
        }
    }

    pub fn balance_formatted(&self) -> String {
        Money::from_str(&*self.balance.to_string(), iso::BRL).unwrap().to_string()
    }
}

impl Account {
    pub fn add_balance(&mut self, value: i32) {
        self.balance += value
    }

    pub fn withdraw_balance(&mut self, value: i32) {
        self.balance -= value
    }
}