use crate::Database;
use quark_common::{Address, H256};
use quark_types::{Account, Body, Header};
use std::collections::HashMap;

pub struct MemDb {
    headers: HashMap<H256, Header>,
    bodies: HashMap<H256, Body>,
    accounts: HashMap<Address, Account>,
    code: HashMap<H256, Vec<u8>>,
    storage: HashMap<H256, H256>,
    nodes: HashMap<H256, H256>,
}

impl Database for MemDb {
    fn header(&self, hash: &H256) -> Option<Header> {
        self.headers.get(&hash).cloned()
    }

    fn insert_header(&mut self, header: Header) -> Option<Header> {
        self.headers.insert(H256::zero(), header)
    }

    fn remove_header(&mut self, hash: &H256) -> Option<Header> {
        self.headers.remove(hash)
    }

    fn body(&self, hash: &H256) -> Option<Body> {
        self.bodies.get(&hash).cloned()
    }

    fn insert_body(&mut self, body: Body) -> Option<Body> {
        self.bodies.insert(H256::zero(), body)
    }

    fn remove_body(&mut self, hash: &H256) -> Option<Body> {
        self.bodies.remove(hash)
    }

    fn account(&self, addr: &Address) -> Option<Account> {
        self.accounts.get(addr).cloned()
    }

    fn insert_account(&mut self, addr: Address, account: Account) -> Option<Account> {
        self.accounts.insert(addr, account)
    }

    fn remove_account(&mut self, addr: &Address) -> Option<Account> {
        self.accounts.remove(addr)
    }

    fn code(&self, hash: &H256) -> Option<Vec<u8>> {
        self.code.get(hash).cloned()
    }

    fn insert_code(&mut self, code: Vec<u8>) -> Option<Vec<u8>> {
        self.code.insert(H256::zero(), code)
    }

    fn storage(&self, _: &Address, key: &H256) -> Option<H256> {
        self.storage.get(key).cloned()
    }

    fn insert_storage(&mut self, _: &Address, key: &H256, val: H256) -> Option<H256> {
        self.storage.insert(key.clone(), val)
    }

    fn node(&self, key: &H256) -> Option<H256> {
        self.nodes.get(key).cloned()
    }

    fn insert_node(&mut self, key: H256, val: H256) -> Option<H256> {
        self.nodes.insert(key, val)
    }

    fn remove_node(&mut self, key: &H256) -> Option<H256> {
        self.nodes.remove(key)
    }
}
