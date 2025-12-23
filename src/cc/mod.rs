use std::collections::HashMap;

use crate::{
    ast::{InnerNode, Node}, err::PgError, lex::Type, op::Op, vm::Value
};

#[derive(Debug, Default)]
pub struct Context {
    global_idx: HashMap<usize, usize>
    global_pool: Vec<Value>,
}

#[derive(Debug)]
pub struct Cc<'cc> {
    buf: Vec<Op<'cc>>,
    ctx: Context,
}

impl<'cc> Cc<'cc> {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(256),
            ctx: Context::default(),
        }
    }

    pub fn compile(&mut self, ast: Node<'cc>) -> Result<(), PgError> {
        let tok = ast.token;
        match ast.inner {
            InnerNode::Atom=> {
                let Some(hash) = tok.hash else {
                    unreachable!()
                };

                let idx = match self.ctx.global_idx.get(&hash) {
                    Some(global_idx) => {global_idx}
                    None => 0,
                };
                self.buf.push(Op::LoadG { dst: 0, idx  })
            }
 InnerNode::Ident =>{}
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
}

#[cfg(test)]
mod cc {
    use crate::{
        ast::{InnerNode, Node},
        cc::Cc,
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
    fn atom() {
        let mut cc = Cc::new();
        let ast = Node {
            token: token!(Type::String("hola")),
            inner: InnerNode::Atom,
        };
        cc.compile(ast).expect("Failed to compile node");

        assert_eq!(cc.buf, vec![Op::LoadG { dst: 0, idx: 0 }],);
    }

    #[test]
    fn ident() {}
}
