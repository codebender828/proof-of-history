use dashmap::DashMap;

use crate::account::Account;

pub struct State {
    accounts: DashMap<String, Account>,
}
