use std::str::FromStr;

use anyhow::anyhow;
use tracing::{debug, instrument, trace, warn};
use tree_sitter::{Parser, Tree};

use crate::javadoc::{node_to_docable, FileContext, JavaDocable, JavaDocableElement};

#[instrument(skip_all)]
pub fn from_sourcecode(sourcecode: &str) -> anyhow::Result<String> {
    let tree = parse_string(sourcecode)?;
    debug!("Getting root node first");
    let root = tree.root_node();
    let mut cursor = root.walk();
    let filecontext = FileContext::from_str(sourcecode)?;

    let children: Vec<JavaDocableElement> = root
        .children(&mut cursor)
        .filter_map(|node| node_to_docable(node, &filecontext))
        .collect();

    debug!("{children:?}");
    let result: Vec<String> = children
        .into_iter()
        .map(|child| match child {
            JavaDocableElement::Class(child) => child.render(0),
            JavaDocableElement::Field(child) => child.render(0),
            JavaDocableElement::Method(child) => child.render(0),
            JavaDocableElement::Constructor(child) => child.render(0),
        })
        .collect();
    let result = result.join("");

    Ok(result)
}

#[instrument(skip_all)]
fn parse_string(sourcecode: &str) -> anyhow::Result<Tree> {
    trace!("Building new parser");
    let mut parser = Parser::new();
    let language = tree_sitter_java::LANGUAGE;
    parser.set_language(&language.into())?;
    trace!("parsing code: {sourcecode:?}");
    let tree = parser
        .parse(sourcecode, None)
        .ok_or_else(|| anyhow!("tree not found"))?;
    trace!("parse was a success");
    Ok(tree)
}
