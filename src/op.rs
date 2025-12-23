use crate::{cc::Cc, vm::BuiltinFn};

#[derive(Debug)]
pub enum Op<'vm> {
    /// Binary operations: lhs = lhs <op> rhs
    Add {
        lhs: u8,
        rhs: u8,
    },
    Sub {
        lhs: u8,
        rhs: u8,
    },
    Mul {
        lhs: u8,
        rhs: u8,
    },
    Div {
        lhs: u8,
        rhs: u8,
    },
    Eq {
        lhs: u8,
        rhs: u8,
    },
    Lt {
        lhs: u8,
        rhs: u8,
    },
    Gt {
        lhs: u8,
        rhs: u8,
    },
    Mov {
        dst: u8,
        src: u8,
    },
    LoadI {
        dst: u8,
        value: i64,
    },
    LoadG {
        dst: u8,
        idx: u32,
    },
    Size {
        dst: u8,
        value: u32,
    },
    Var {
        hash: u64,
        src: u8,
    },
    LoadV {
        hash: u64,
        dst: u8,
    },
    New {
        dst: u8,
        size: u8,
        new_type: New,
    },
    Append {
        container: u8,
        src: u8,
    },
    Len {
        dst: u8,
        src: u8,
    },
    Idx {
        dst: u8,
        container: u8,
        index: u8,
    },
    Jmp {
        target: usize,
    },
    JmpF {
        cond: u8,
        target: usize,
    },
    Call {
        func: u16,
        args_start: u8,
        args_len: u8,
    },
    Ret {
        /// used for peephole optimisation, merging multiple RET into a single RET with a count
        times: u8,
    },
    Sys {
        ptr: BuiltinFn<'vm>,
        args_start: u8,
        args_len: u8,
    },
}

#[derive(Debug)]
pub enum New {
    Object,
    Array,
}

impl<'vm> Op<'vm> {
    pub fn disassemble(base: usize, cc: &Cc, bytecode: &[Op<'vm>]) {
        for (pc, op) in bytecode.iter().enumerate() {
            print!("{:04}: ", pc);
            match op {
                Op::Add { lhs, rhs } => println!("ADD r{}, r{}", lhs, rhs),
                Op::Sub { lhs, rhs } => println!("SUB r{}, r{}", lhs, rhs),
                Op::Mul { lhs, rhs } => println!("MUL r{}, r{}", lhs, rhs),
                Op::Div { lhs, rhs } => println!("DIV r{}, r{}", lhs, rhs),
                Op::Eq { lhs, rhs } => println!("EQ r{}, r{}", lhs, rhs),
                Op::Lt { lhs, rhs } => println!("LT r{}, r{}", lhs, rhs),
                Op::Gt { lhs, rhs } => println!("GT r{}, r{}", lhs, rhs),
                Op::Mov { dst, src } => println!("MOV r{}, r{}", dst, src),
                Op::LoadI { dst, value } => println!("LOADI r{}, {}", dst, value),
                Op::LoadG { dst, idx } => println!("LOADG r{}, global[{}]", dst, idx),
                Op::Size { dst, value } => println!("SIZE r{}, {}", dst, value),
                Op::Var { hash, src } => println!("VAR hash={} r{}", hash, src),
                Op::LoadV { hash, dst } => println!("LOADV r{} hash={}", dst, hash),
                Op::New {
                    dst,
                    size,
                    new_type,
                } => {
                    println!("NEW r{} {:?}[size={}]", dst, new_type, size);
                }
                Op::Append { container, src } => println!("APPEND r{} r{}", container, src),
                Op::Len { dst, src } => println!("LEN r{} r{}", dst, src),
                Op::Idx {
                    dst,
                    container,
                    index,
                } => {
                    println!("IDX   r{} r{}[r{}]", dst, container, index)
                }
                Op::Jmp { target } => println!("JMP {}", target),
                Op::JmpF { cond, target } => println!("JMPF r{} {}", cond, target),
                Op::Call {
                    func,
                    args_start,
                    args_len,
                } => {
                    println!(
                        "CALL {} r{}..r{}",
                        func,
                        args_start,
                        args_start + args_len - 1
                    );
                }
                Op::Ret { times } => println!("RET *{}", times),
                Op::Sys {
                    ptr,
                    args_start,
                    args_len,
                } => {
                    println!(
                        "SYS 0x{} r{}..r{}",
                        *ptr as usize - base,
                        args_start,
                        args_start + args_len - 1
                    )
                }
            }
        }
    }
}
