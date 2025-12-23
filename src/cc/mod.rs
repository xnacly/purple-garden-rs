use std::collections::HashMap;

use crate::{
    ast::{InnerNode, Node},
    err::PgError,
    lex::Type,
    op::Op,
    vm::{Value, Vm},
};

/// Compile time Value representation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Const<'c> {
    False,
    True,
    Int(i64),
    Double(u64),
    Str(&'c str),
}

#[derive(Debug, Default)]
pub struct Context<'ctx> {
    globals: HashMap<Const<'ctx>, usize>,
    globals_vec: Vec<Const<'ctx>>,
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

#[derive(Debug)]
pub struct Cc<'cc> {
    buf: Vec<Op<'cc>>,
    ctx: Context<'cc>,
}

impl<'cc> Cc<'cc> {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(256),
            ctx: {
                let mut ctx = Context::default();
                ctx.intern(Const::False);
                ctx.intern(Const::True);
                ctx
            },
        }
    }

    pub const GLOBAL_FALSE: u32 = 0;
    pub const GLOBAL_TRUE: u32 = 1;

    pub fn compile(&mut self, ast: Node<'cc>) -> Result<(), PgError> {
        match ast.inner {
            InnerNode::Atom => {
                match &ast.token.t {
                    Type::Integer(inner) => self.buf.push(Op::LoadI {
                        dst: 0,
                        value: inner.parse().map_err(|msg: std::num::ParseIntError| {
                            PgError::with_msg(msg.to_string(), &ast.token)
                        })?,
                    }),
                    Type::String(inner) => self.buf.push(Op::LoadG {
                        dst: 0,
                        idx: self.ctx.intern(Const::Str(inner)),
                    }),
                    Type::Double(inner) => {
                        let bits = inner.parse::<f64>().unwrap().to_bits();
                        self.buf.push(Op::LoadG {
                            dst: 0,
                            idx: self.ctx.intern(Const::Double(bits)),
                        })
                    }
                    Type::True => {
                        let idx = Self::GLOBAL_TRUE;
                        self.buf.push(Op::LoadG { dst: 0, idx });
                    }
                    Type::False => {
                        let idx = Self::GLOBAL_FALSE;
                        self.buf.push(Op::LoadG { dst: 0, idx });
                    }
                    _ => unreachable!(),
                };
            }
            InnerNode::Ident => {}
            InnerNode::Bin { lhs, rhs } => {}
            InnerNode::Array { members } => {}
            InnerNode::Object { pairs } => {}
            InnerNode::Let { rhs } => {}
            InnerNode::Fn { args, body } => {}
            InnerNode::Match { cases, default } => {}
            InnerNode::Call { args } => {}
            InnerNode::Path { members, leaf } => {}
        };
        Ok(())
    }

    pub fn finalize(self) -> Vm<'cc> {
        let mut v = Vm {
            ..Default::default()
        };
        v.bytecode = self.buf;
        v.globals = self.ctx.globals_vec.into_iter().map(Value::from).collect();
        v
    }
}

#[cfg(test)]
mod cc {
    use crate::{
        ast::{InnerNode, Node},
        cc::{Cc, Const},
        lex::{Token, Type},
        op::Op,
    };

    macro_rules! node {
        ($expr:expr) => {
            Node {
                token: token!(Type::String("hola")),
                inner: $expr,
            }
        };
    }

    macro_rules! token {
        ($expr:expr) => {
            Token {
                line: 0,
                col: 0,
                t: $expr,
            }
        };
    }

    #[test]
    fn atom_false() {
        let mut cc = Cc::new();
        let ast = Node {
            token: token!(Type::False),
            inner: InnerNode::Atom,
        };

        let _ = cc.compile(ast).expect("Failed to compile node");
        let expected_idx: usize = 0;
        assert_eq!(
            cc.buf,
            vec![Op::LoadG {
                dst: 0,
                idx: expected_idx as u32
            }],
        );
        assert_eq!(cc.ctx.globals_vec[expected_idx], Const::False);
    }

    #[test]
    fn atom_true() {
        let mut cc = Cc::new();
        let ast = Node {
            token: token!(Type::True),
            inner: InnerNode::Atom,
        };

        let _ = cc.compile(ast).expect("Failed to compile node");
        let expected_idx: usize = 1;
        assert_eq!(
            cc.buf,
            vec![Op::LoadG {
                dst: 0,
                idx: expected_idx as u32
            }],
        );
        assert_eq!(cc.ctx.globals_vec[expected_idx], Const::True);
    }

    #[test]
    fn atom_string() {
        let mut cc = Cc::new();
        let ast = Node {
            token: token!(Type::String("hola")),
            inner: InnerNode::Atom,
        };

        let _ = cc.compile(ast).expect("Failed to compile node");
        let expected_idx: usize = 2;
        assert_eq!(
            cc.buf,
            vec![Op::LoadG {
                dst: 0,
                idx: expected_idx as u32
            }],
        );
        assert_eq!(cc.ctx.globals_vec[expected_idx], Const::Str("hola"));
    }

    #[test]
    fn atom_int() {
        let mut cc = Cc::new();
        let ast = Node {
            token: token!(Type::Integer("25")),
            inner: InnerNode::Atom,
        };
        let _ = cc.compile(ast).expect("Failed to compile node");
        assert_eq!(cc.buf, vec![Op::LoadI { dst: 0, value: 25 }],);
    }

    #[test]
    fn atom_double() {
        let mut cc = Cc::new();
        let ast = Node {
            token: token!(Type::Double("3.1415")),
            inner: InnerNode::Atom,
        };
        let _ = cc.compile(ast).expect("Failed to compile node");
        let expected_idx: usize = 2;
        assert_eq!(
            cc.buf,
            vec![Op::LoadG {
                dst: 0,
                idx: expected_idx as u32
            }],
        );
        assert_eq!(
            cc.ctx.globals_vec[expected_idx],
            Const::Double((3.1415_f64).to_bits())
        );
    }
}
