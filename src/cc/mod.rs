use crate::{
    ast::{InnerNode, Node},
    err::PgError,
    op::Op,
};

#[derive(Debug, Default)]
pub struct Context {}

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
        match ast.inner {
            InnerNode::Atom | InnerNode::Ident => todo!(),
            InnerNode::Bin { lhs, rhs } => todo!(),
            InnerNode::Array { members } => todo!(),
            InnerNode::Object { pairs } => todo!(),
            InnerNode::Let { rhs } => todo!(),
            InnerNode::Fn { args, body } => todo!(),
            InnerNode::Match { cases, default } => todo!(),
            InnerNode::Call { args } => todo!(),
            InnerNode::Path { members, leaf } => todo!(),
        };
        Ok(())
    }
}
