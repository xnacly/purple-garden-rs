use std::collections::HashMap;

use crate::{Todo, gc::Gc};

pub enum Value {
    Int(i64),
    Double(f64),
    /// a view into the bytes of the interpreters input, compile time strings
    Str(&'static str),
    String(String),
    Arr(Gc<[Value]>),
    Obj(Gc<Todo>),
}

struct Frame<'frame> {
    variables: HashMap<&'frame str, Value>,
    return_to: usize,
    prev: Box<Frame<'frame>>,
}

pub struct Vm<'vm> {
    registers: [Option<Value>; 32],
    pc: usize,
    frame: Frame<'vm>,
}

pub type BuiltinFn<'vm> = fn(&mut Vm<'vm>, &[Value]);
