use javadoc2adoc_types::BlockComment;
use tracing::{debug, trace};
use tree_sitter::Node;

use super::FileContext;

pub fn find_block_comment<'a>(
    node: Node<'a>,
    context: &'a FileContext,
) -> Option<BlockComment<'a>> {
    let sibling = node.prev_sibling();
    match sibling {
        Some(sibling) => {
            let name = sibling.grammar_name();
            if name == "block_comment" {
                debug!("Found a sibling block_comment");
                let start = sibling.range();
                let start = start.start_byte;
                let end = start + 3;
                let start = context.source_for_start_end(start, end);

                trace!("Comment start: {start}");
                if start == "/**" {
                    debug!("Is javadoc");
                    let comment = BlockComment::new(sibling, context);
                    Some(comment)
                } else {
                    None
                }
            } else {
                None
            }
        }
        None => None,
    }
}
