use crate::op::Op;

/// this is used to display static variables and functions in the standard libary in disassembly
static STATIC_BASE: u8 = 0;

mod ast;
mod cc;
/// pretty print errors
mod err;
/// simple mark and sweep garbage collector, will be replaced by a manchester garbage collector in
/// the future
mod gc;
mod lex;
/// purple garden bytecode virtual machine operations
mod op;
mod parser;
/// register based virtual machine
mod vm;

type Todo = ();

// TODO:
// - port pg cli to serde
// - port frontend (lexer, parser)
//      - port tokens
//      - port ast
// - port cc
// - port vm fully
// - port gc
// - implement very good errors
fn main() {
    let bytecode: Vec<Op> = vec![
        Op::LoadI { dst: 0, value: 10 },
        Op::LoadI { dst: 1, value: 32 },
        Op::Add { lhs: 0, rhs: 1 },
        Op::Var {
            hash: 0x123,
            src: 0,
        },
        Op::LoadV {
            hash: 0x123,
            dst: 2,
        },
        Op::LoadG { dst: 3, idx: 1 },
        Op::Call {
            func: 1,
            args_start: 0,
            args_len: 2,
        },
        Op::Sys {
            ptr: |_, _| {},
            args_start: 0,
            args_len: 1,
        },
        Op::Ret { times: 1 },
    ];

    Op::disassemble(
        &STATIC_BASE as *const u8 as usize,
        &cc::Cc::new(),
        &bytecode,
    );
}
