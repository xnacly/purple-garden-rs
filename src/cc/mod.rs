use std::{
    hash::{DefaultHasher, Hash, Hasher},
    num,
};

mod ctx;
mod reg;

use crate::{
    ast::{InnerNode, Node},
    cc::{ctx::Context, reg::RegisterAllocator},
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

#[derive(Debug)]
pub struct Cc<'cc> {
    buf: Vec<Op<'cc>>,
    ctx: Context<'cc>,
    register: RegisterAllocator,
    hasher: DefaultHasher,
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
            register: RegisterAllocator::new(),
            hasher: DefaultHasher::new(),
        }
    }

    pub const GLOBAL_FALSE: u32 = 0;
    pub const GLOBAL_TRUE: u32 = 1;

    fn load_const(&mut self, c: Const<'cc>) -> u8 {
        let r = self.register.alloc();
        self.buf.push(Op::LoadG {
            dst: r,
            idx: self.ctx.intern(c),
        });
        r
    }

    fn hash(&mut self, to_hash: impl Hash) -> u64 {
        to_hash.hash(&mut self.hasher);
        self.hasher.finish()
    }

    /// compile is a simple wrapper around self.cc to make sure all registers are deallocated after
    /// their lifetime ends
    fn compile(&mut self, ast: Node<'cc>) -> Result<(), PgError> {
        if let Some(register) = self.cc(ast)? {
            self.register.free(register);
        }

        Ok(())
    }

    pub fn cc(&mut self, ast: Node<'cc>) -> Result<Option<u8>, PgError> {
        Ok(match ast.inner {
            InnerNode::Atom => {
                let constant = match &ast.token.t {
                    Type::Integer(s) => {
                        let value = s.parse().map_err(|e: num::ParseIntError| {
                            PgError::with_msg(e.to_string(), &ast.token)
                        })?;

                        let r = self.register.alloc();
                        self.buf.push(Op::LoadI { dst: r, value });

                        // early bail, since we do LoadG for the other values
                        return Ok(Some(r));
                    }
                    Type::Double(s) => Const::Double(
                        s.parse::<f64>()
                            .map_err(|e: num::ParseFloatError| {
                                PgError::with_msg(e.to_string(), &ast.token)
                            })?
                            .to_bits(),
                    ),
                    Type::String(s) => Const::Str(s),
                    Type::True => Const::True,
                    Type::False => Const::False,
                    _ => unreachable!(
                        "This is considered an impossible path, InnerNode::Atom can only have Type::{{Integer, Double, String, True, False}}"
                    ),
                };

                Some(self.load_const(constant))
            }
            InnerNode::Ident => {
                let Type::Ident(name) = ast.token.t else {
                    unreachable!("InnerNode::Ident");
                };
                let r = self.register.alloc();
                let hash = self.hash(name);
                self.buf.push(Op::LoadV { dst: r, hash });
                Some(r)
            }
            _ => todo!("{:?}", ast),
        })
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
    use std::hash::{Hash, Hasher};

    use crate::{
        ast::{InnerNode, Node},
        cc::{Cc, Const},
        lex::{Token, Type},
        op::Op,
    };

    // macro_rules! node {
    //     ($expr:expr) => {
    //         Node {
    //             token: token!(Type::String("hola")),
    //             inner: $expr,
    //         }
    //     };
    // }

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

    #[test]
    fn atom_ident() {
        let mut cc = Cc::new();
        let name = "thisisavariablename";
        let ast = Node {
            token: token!(Type::Ident(name)),
            inner: InnerNode::Ident,
        };
        let mut s = std::hash::DefaultHasher::new();
        name.hash(&mut s);
        let hash = s.finish();
        let _ = cc.compile(ast).expect("Failed to compile node");
        let expected_idx: usize = 2;
        assert_eq!(cc.buf, vec![Op::LoadV { dst: 0, hash }],);
    }
}
