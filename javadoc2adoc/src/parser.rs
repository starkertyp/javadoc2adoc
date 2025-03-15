use anyhow::anyhow;
use tracing::{instrument, trace};
use tree_sitter::{Parser, Tree};

#[instrument(skip_all)]
pub fn parse_string(sourcecode: &str) -> anyhow::Result<Tree> {
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
