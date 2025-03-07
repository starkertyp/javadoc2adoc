use std::fmt;

use tracing::debug;
use tree_sitter::Node;

use crate::javadoc::comment::find_block_comment;

use super::{comment::BlockComment, get_string_of_node, FileContext, JavaDocable};

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
        let nodetype = get_string_of_node(&nodetype, &ctx.0);
        let name = get_string_of_node(&name, &ctx.0);
        format!("{nodetype} {name}")
    }

    fn get_comment(&self) -> &'a BlockComment {
        &self.comment
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
