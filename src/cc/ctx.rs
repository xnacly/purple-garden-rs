use std::collections::HashMap;

use crate::cc::Const;

#[derive(Debug, Default)]
pub struct Context<'ctx> {
    pub globals: HashMap<Const<'ctx>, usize>,
    pub globals_vec: Vec<Const<'ctx>>,
}

impl<'ctx> Context<'ctx> {
    pub fn intern(&mut self, constant: Const<'ctx>) -> u32 {
        if let Some(&idx) = self.globals.get(&constant) {
            return idx as u32;
        }

        let idx = self.globals_vec.len();
        self.globals_vec.push(constant);
        self.globals.insert(constant, idx);
        idx as u32
    }
}
