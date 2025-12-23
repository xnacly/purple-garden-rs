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
        let register = self.cc(ast)?;
        self.register.free(register);

        Ok(())
    }

    pub fn cc(&mut self, ast: Node<'cc>) -> Result<u8, PgError> {
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
                        return Ok(r);
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

                self.load_const(constant)
            }
            InnerNode::Ident => {
                let Type::Ident(name) = ast.token.t else {
                    unreachable!("InnerNode::Ident");
                };
                let r = self.register.alloc();
                let hash = self.hash(name);
                self.buf.push(Op::LoadV { dst: r, hash });
                r
            }
            InnerNode::Bin { lhs, rhs } => {
                let lhs = self.cc(*lhs)?;
                let rhs = self.cc(*rhs)?;

                let dst = self.register.alloc();
                self.buf.push(match ast.token.t {
                    Type::Plus => Op::Add { dst, lhs, rhs },
                    Type::Minus => Op::Sub { dst, lhs, rhs },
                    Type::Asteriks => Op::Mul { dst, lhs, rhs },
                    Type::Slash => Op::Div { dst, lhs, rhs },
                    Type::LessThan => Op::Lt { dst, lhs, rhs },
                    Type::GreaterThan => Op::Gt { dst, lhs, rhs },
                    Type::Equal => Op::Eq { dst, lhs, rhs },
                    _ => unreachable!(),
                });

                self.register.free(lhs);
                self.register.free(rhs);
                dst
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

    macro_rules! node {
        ($token:expr, $inner:expr) => {
            Node {
                token: $token,
                inner: $inner,
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

    #[test]
    fn atom_bin() {
        use crate::lex::Type::*;
        use crate::op::Op::*;

        let tests: Vec<(Type, fn(u8, u8, u8) -> Op<'static>)> = vec![
            (Plus, |dst, lhs, rhs| Add { dst, lhs, rhs }),
            (Minus, |dst, lhs, rhs| Sub { dst, lhs, rhs }),
            (Asteriks, |dst, lhs, rhs| Mul { dst, lhs, rhs }),
            (Slash, |dst, lhs, rhs| Div { dst, lhs, rhs }),
            (Equal, |dst, lhs, rhs| Eq { dst, lhs, rhs }),
            (LessThan, |dst, lhs, rhs| Lt { dst, lhs, rhs }),
            (GreaterThan, |dst, lhs, rhs| Gt { dst, lhs, rhs }),
        ];

        for (token_type, make_op) in tests {
            let mut cc = Cc::new();

            let ast = Node {
                token: token!(token_type.clone()),
                inner: InnerNode::Bin {
                    lhs: Box::new(node!(token!(Type::Integer("45")), InnerNode::Atom)),
                    rhs: Box::new(node!(token!(Type::Integer("45")), InnerNode::Atom)),
                },
            };

            let _ = cc.compile(ast).expect("Failed to compile node");

            let expected_op = make_op(2, 0, 1);

            assert_eq!(
                cc.buf,
                vec![
                    Op::LoadI { dst: 0, value: 45 },
                    Op::LoadI { dst: 1, value: 45 },
                    expected_op,
                ],
                "Failed for operator: {:?}",
                token_type
            );
        }
    }
}
