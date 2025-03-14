use core::fmt;

use tracing::{debug, instrument, trace};
use tree_sitter::Node;

use super::FileContext;

#[derive(Debug)]
pub struct BlockComment<'a> {
    node: Node<'a>,
    context: &'a FileContext,
}

impl<'a> BlockComment<'a> {
    pub fn new(node: Node<'a>, context: &'a FileContext) -> Self {
        Self { node, context }
    }
}

impl fmt::Display for BlockComment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node = self.node;
        let range = node.range();
        let ctx = self.context;
        let source = &ctx.0;
        let source = &source[range.start_byte..range.end_byte];
        let doc = javadoc_to_adoc(source);
        writeln!(f, "{doc}")
    }
}

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
                let sourcecode = &context.0;
                let start = sibling.range();
                let start = start.start_byte;
                let end = start + 3;
                let start = &sourcecode[start..end];
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

#[instrument(skip_all)]
fn javadoc_to_adoc(source: &str) -> String {
    let lines: Vec<String> = source
        .lines()
        .map(|line| {
            let line = line.trim();
            let line = line.replace("<p>", "");
            if line.starts_with("/**") {
                trace!("Stripping '/**' from {line:?}");
                return line.strip_prefix("/**").unwrap().trim_start().to_owned();
            } else if line.starts_with("*/") {
                trace!("Stripping '*/' from {line:?}");
                return line.strip_prefix("*/").unwrap().trim_start().to_owned();
            } else if line.starts_with("*") {
                trace!("Stripping '*' from {line:?}");
                return line.strip_prefix("*").unwrap().trim_start().to_owned();
            } else {
                return line;
            }
        })
        .map(|line| {
            if line.starts_with("@") {
                trace!("Found an @ annotation in line {line:?}");
                let line = line.strip_prefix("@").unwrap();
                let first_space = line.find(" ");
                if let Some(first_space) = first_space {
                    let head = &line[..first_space];
                    let tail = &line[first_space..];
                    trace!("Head: {head:?} | Tail: {tail:?}");
                    format!("{head}::{tail}")
                } else {
                    line.to_string()
                }
            } else {
                line.to_string()
            }
        })
        .collect();

    lines.join("\n")
}

#[cfg(test)]
mod javadoc_to_adoc_tests {
    use super::*;

    #[test]
    fn removes_p_tags() {
        let input = "* <p> some cool documentation";
        let result = javadoc_to_adoc(input);
        assert_eq!(result, "some cool documentation");
    }
}
