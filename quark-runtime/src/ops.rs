use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Clone, Copy, FromPrimitive, IntoPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Jmp,
    Jz,
    Read,
    Write,
    MemRead,
    MemWrite,

    #[num_enum(default)]
    Invalid,
}
