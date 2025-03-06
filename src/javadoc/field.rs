use std::fmt;

use tracing::debug;
use tree_sitter::Node;

use crate::javadoc::comment::find_block_comment;

use super::{comment::BlockComment, FileContext, JavaDocable};

#[derive(Debug)]
pub struct Field<'a> {
    comment: BlockComment<'a>,
    node: Node<'a>,
    context: &'a FileContext,
}

impl<'a> JavaDocable<'a> for Field<'a> {
    fn new(ctx: &'a super::FileContext, node: Node<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        let comment = find_block_comment(node, ctx);
        if let Some(comment) = comment {
            Some(Self {comment, node, context: ctx})
        } else {
            debug!("Found a field but no block comment, skipping");
            None
        }
    }

    fn has_children(&self) -> bool {
        false
    }

    fn get_node(&self) -> tree_sitter::Node<'_> {
        self.node
    }

    fn get_context(&self) -> &'a super::FileContext {
        self.context
    }

    fn get_name(&self) -> String {
        todo!()
    }
}

impl fmt::Display for Field<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.get_name();
        writeln!(f, "")?;
        writeln!(f, "=== {}", name)?;
        writeln!(f, "")?;
        writeln!(f, "{}", self.comment)
    }
}
