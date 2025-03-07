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
            Some(Self {
                comment,
                node,
                context: ctx,
            })
        } else {
            debug!("Found a field but no block comment, skipping");
            None
        }
    }
    fn get_node(&self) -> tree_sitter::Node<'_> {
        self.node
    }

    fn get_context(&self) -> &'a super::FileContext {
        self.context
    }

    fn get_name(&self) -> String {
        let node = self.get_node();
        let ctx = self.get_context();
        let declarator = node.child_by_field_name("declarator").unwrap();
        let nodetype = node.child_by_field_name("type").unwrap();
        let name = declarator.child_by_field_name("name").unwrap();
        let nodetype = ctx.source_for_range(&nodetype.range());
        let name = ctx.source_for_range(&name.range());
        format!("{nodetype} {name}")
    }

    fn get_comment(&self) -> &'a BlockComment {
        &self.comment
    }
}
