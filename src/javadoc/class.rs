use tracing::debug;
use tree_sitter::Node;

use super::{comment::{find_block_comment, BlockComment}, FileContext, JavaDocable};

#[derive(Debug)]
pub struct Class<'a> {
    comment: BlockComment<'a>,
    node: Node<'a>,
    context: &'a FileContext,
}

impl<'a> JavaDocable<'a> for Class<'a> {
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
            debug!("Found a class but no block comment, skipping");
            None
        }
    }

    fn has_children(&self) -> bool {
        true
    }

    fn get_node(&self) -> Node<'_> {
        self.node
    }

    fn get_context(&self) -> &'a FileContext {
        self.context
    }

    fn get_name(&self) -> String {
        "A class".to_string()
    }
}
