use crate::env::Environment;
use ethereum_types::U256;

pub struct Context<'a> {
    pub env: &'a Environment,
    pub pc: u64,
    pub start_gas: u64,
    pub stack: Vec<U256>,
    pub memory: Vec<u8>,
}

impl<'a> Context<'a> {
    pub fn pc(&self) -> usize {
        self.pc as usize
    }
}
