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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_create() {
        let user = User {
            name: "Jorge".to_string(),
            last_name: "Carlos".to_string()
        };
        let account = Account::new(user, 1500);
        assert_eq!(account.user.full_name(), "Jorge Carlos");
        assert_eq!(account.balance, 1500);
        assert_eq!(account.balance_formatted(), "R$1,500.00")
    }

    #[test]
    fn test_account_add_balance() {
        let user = User {
            name: "Jorge".to_string(),
            last_name: "Carlos".to_string()
        };
        let mut account = Account::new(user, 1500);
        account.add_balance(500);
        assert_eq!(account.balance, 2000);
    }

    #[test]
    fn test_account_withdraw_balance() {
        let user = User {
            name: "Jorge".to_string(),
            last_name: "Carlos".to_string()
        };
        let mut account = Account::new(user, 1500);
        account.withdraw_balance(500);
        assert_eq!(account.balance, 1000);
    }
}