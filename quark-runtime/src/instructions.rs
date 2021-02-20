use crate::{ctx::Context, error::Error};

pub fn add(ctx: &mut Context) -> Result<(), Error> {
    let x = ctx.stack.pop().ok_or(Error::StackUnderflow)?;
    let y = ctx.stack.pop().ok_or(Error::StackUnderflow)?;
    ctx.stack.push(x + y);
    Ok(())
}

pub fn sub(ctx: &mut Context) -> Result<(), Error> {
    let x = ctx.stack.pop().ok_or(Error::StackUnderflow)?;
    let y = ctx.stack.pop().ok_or(Error::StackUnderflow)?;
    ctx.stack.push(x - y);
    Ok(())
}

pub fn mul(ctx: &mut Context) -> Result<(), Error> {
    let x = ctx.stack.pop().ok_or(Error::StackUnderflow)?;
    let y = ctx.stack.pop().ok_or(Error::StackUnderflow)?;
    ctx.stack.push(x * y);
    Ok(())
}

pub fn jmp(ctx: &mut Context) -> Result<(), Error> {
    let dest = ctx.stack.pop().ok_or(Error::StackUnderflow)?;
    ctx.pc = dest.as_u64();
    Ok(())
}

pub fn jz(ctx: &mut Context) -> Result<(), Error> {
    let dest = ctx.stack.pop().ok_or(Error::StackUnderflow)?;
    ctx.pc = dest.as_u64();
    Ok(())
}

pub fn read(ctx: &mut Context) -> Result<(), Error> {
    unimplemented!()
}

pub fn write(ctx: &mut Context) -> Result<(), Error> {
    unimplemented!()
}

pub fn memread(ctx: &mut Context) -> Result<(), Error> {
    unimplemented!()
}

pub fn memwrite(ctx: &mut Context) -> Result<(), Error> {
    unimplemented!()
}

pub fn invalid(ctx: &mut Context) -> Result<(), Error> {
    unimplemented!()
}
