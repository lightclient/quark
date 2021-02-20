use crate::{ctx::Context, env::Environment, instructions, ops::Op};

pub fn execute<'a>(env: &'a Environment, code: &[u8]) {
    let mut ctx = Context {
        env,
        pc: 0,
        start_gas: 0,
        stack: Vec::new(),
        memory: Vec::new(),
    };

    while ctx.pc() < code.len() {
        match Op::from(code[ctx.pc()]) {
            Op::Add => instructions::add(&mut ctx),
            Op::Sub => instructions::sub(&mut ctx),
            Op::Mul => instructions::mul(&mut ctx),
            Op::Jmp => instructions::jmp(&mut ctx),
            Op::Jz => instructions::jz(&mut ctx),
            Op::Read => instructions::read(&mut ctx),
            Op::Write => instructions::write(&mut ctx),
            Op::MemRead => instructions::memread(&mut ctx),
            Op::MemWrite => instructions::memwrite(&mut ctx),
            Op::Invalid => instructions::invalid(&mut ctx),
        }
        .expect("op to evaluate properly");
    }

    unimplemented!()
}
