use tracing::debug;
use tree_sitter::Node;

use super::{comment::{find_block_comment, BlockComment}, FileContext, JavaDocable};

#[derive(Debug)]
pub struct Constructor<'a> {
    comment: BlockComment<'a>,
    node: Node<'a>,
    context: &'a FileContext,
}

impl<'a> JavaDocable<'a> for Constructor<'a> {
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

    fn get_node(&self) -> Node<'_> {
    self.node}

    fn get_context(&self) -> &'a FileContext {
    self.context}

    fn get_comment(&self) -> &'a BlockComment {
    &self.comment}

    fn get_name(&self) -> String {
        let node = self.get_node();
        let ctx = self.get_context();
        let name = node.child_by_field_name("name").unwrap();
        let params = node.child_by_field_name("parameters").unwrap();
        let name = ctx.source_for_range(&name.range());
        let params = ctx.source_for_range(&params.range());
        format!("{name} {params}")
    }
}
