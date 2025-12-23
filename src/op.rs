use crate::vm::BuiltinFn;

#[derive(Debug)]
#[allow(unpredictable_function_pointer_comparisons)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Op<'vm> {
    Add {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },
    Sub {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },
    Mul {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },
    Div {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },
    Eq {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },
    Lt {
        dst: u8,
        lhs: u8,
        rhs: u8,
    },
    Gt {
        dst: u8,
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
    Let {
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

#[derive(Debug, PartialEq, Eq)]
pub enum New {
    Object,
    Array,
}
