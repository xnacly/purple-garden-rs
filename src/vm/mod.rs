use std::collections::HashMap;

mod value;

use crate::op::Op;
pub use crate::vm::value::Value;

#[derive(Default, Debug)]
pub struct Frame<'frame> {
    variables: HashMap<&'frame str, Value<'frame>>,
    return_to: usize,
    prev: Option<Box<Frame<'frame>>>,
}

#[derive(Default, Debug)]
pub struct Vm<'vm> {
    pub registers: [Option<Value<'vm>>; 32],
    pub pc: usize,
    pub frame: Frame<'vm>,
    pub bytecode: Vec<Op<'vm>>,
    pub globals: Vec<Value<'vm>>,
}

pub type BuiltinFn<'vm> = fn(&mut Vm<'vm>, &[Value]);
