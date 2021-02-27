mod memdb;
pub use memdb::MemDb;

use quark_common::{Address, H256};
use quark_types::{Account, Body, Header};

pub trait Database {
    fn header(&self, hash: &H256) -> Option<Header>;
    fn insert_header(&mut self, header: Header) -> Option<Header>;
    fn remove_header(&mut self, hash: &H256) -> Option<Header>;

    fn body(&self, hash: &H256) -> Option<Body>;
    fn insert_body(&mut self, body: Body) -> Option<Body>;
    fn remove_body(&mut self, hash: &H256) -> Option<Body>;

    fn account(&self, addr: &Address) -> Option<Account>;
    fn insert_account(&mut self, addr: Address, account: Account) -> Option<Account>;
    fn remove_account(&mut self, addr: &Address) -> Option<Account>;

    fn code(&self, hash: &H256) -> Option<Vec<u8>>;
    fn insert_code(&mut self, code: Vec<u8>) -> Option<Vec<u8>>;

    fn storage(&self, addr: &Address, key: &H256) -> Option<H256>;
    fn insert_storage(&mut self, addr: &Address, key: &H256, val: H256) -> Option<H256>;

    fn node(&self, key: &H256) -> Option<H256>;
    fn insert_node(&mut self, key: H256, val: H256) -> Option<H256>;
    fn remove_node(&mut self, key: &H256) -> Option<H256>;
}
