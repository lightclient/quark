use quark_common::H160;

pub struct Environment {
    caller: u64,
    address: H160,
    number: u64,
    gas_limit: u64,
}
