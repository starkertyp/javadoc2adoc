use javadoc2adoc_macros::{default_javadocable_fields, DefaultJavaDocable};
use javadoc2adoc_types::DefaultJavaDocable;
use tracing::debug;
use tree_sitter::Node;

use crate::javadoc::comment::find_block_comment;

use super::JavaDocable;

#[default_javadocable_fields]
#[derive(Debug, DefaultJavaDocable)]
pub struct Field<'a> {}

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
}
