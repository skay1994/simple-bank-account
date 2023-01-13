use crate::structs::User;

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
}