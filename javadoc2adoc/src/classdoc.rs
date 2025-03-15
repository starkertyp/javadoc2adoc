use std::str::FromStr;

use javadoc2adoc_types::FileContext;
use tracing::{debug, instrument, warn};

use crate::{
    javadoc::{node_to_docable, JavaDocable, JavaDocableElement},
    parser::parse_string,
};

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
            JavaDocableElement::Interface(child) => child.render(0),
        })
        .collect();
    let result = result.join("");

    Ok(result)
}
