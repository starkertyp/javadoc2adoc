use javadoc2adoc_macros::{default_javadocable_fields, DefaultJavaDocable};
use javadoc2adoc_types::DefaultJavaDocable;
use tracing::debug;
use tree_sitter::Node;

use super::{comment::find_block_comment, FileContext, JavaDocable};

#[default_javadocable_fields]
#[derive(Debug, DefaultJavaDocable)]
pub struct Method<'a> {}

impl<'a> JavaDocable<'a> for Method<'a> {
    fn new(ctx: &'a FileContext, node: Node<'a>) -> Option<Self>
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
        let nodetype = node.child_by_field_name("type").unwrap();
        let name = node.child_by_field_name("name").unwrap();
        let params = node.child_by_field_name("parameters").unwrap();
        let nodetype = ctx.source_for_range(&nodetype.range());
        let name = ctx.source_for_range(&name.range());
        let params = ctx.source_for_range(&params.range());

        format!("{nodetype} {name} {params}")
    }
}
