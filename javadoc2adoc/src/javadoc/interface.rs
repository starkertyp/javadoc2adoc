use rust_i18n::t;
use tracing::debug;
use tree_sitter::Node;

use crate::javadoc::{method::Method, prefix_hashes};

use javadoc2adoc_macros::default_javadocable_fields;

use super::{
    comment::{find_block_comment, BlockComment},
    node_to_docable, FileContext, JavaDocable, JavaDocableElement,
};

#[default_javadocable_fields]
#[derive(Debug)]
pub struct Interface<'a> {
    children: Vec<JavaDocableElement<'a>>,
}

impl<'a> JavaDocable<'a> for Interface<'a> {
    fn new(ctx: &'a FileContext, node: Node<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        let comment = find_block_comment(node, ctx);
        let mut cursor = node.walk();
        let body = node.child_by_field_name("body").unwrap();

        let mut children: Vec<JavaDocableElement> = vec![];
        for child in body.children(&mut cursor) {
            if let Some(docable) = node_to_docable(child, ctx) {
                children.push(docable);
            }
        }

        if let Some(comment) = comment {
            Some(Self {
                comment,
                node,
                context: ctx,
                children,
            })
        } else {
            debug!("Found an interface but no block comment, skipping");
            None
        }
    }

    fn get_node(&self) -> Node<'_> {
        self.node
    }

    fn get_context(&self) -> &'a FileContext {
        self.context
    }

    fn get_comment(&self) -> &'a BlockComment {
        &self.comment
    }

    fn get_name(&self) -> String {
        let node = self.get_node();
        let name = node.child_by_field_name("name").unwrap();
        let ctx = self.context;
        let name = ctx.source_for_range(&name.range());
        name.to_owned()
    }
    
    fn render(&'a self, level: u8) -> String {
        let prefix_hashes = prefix_hashes(level);
        let name = self.get_name();
        let headline = format!("{prefix_hashes} {name}");
        let content = self.get_comment();
        let content = format!("{content}");

        let mut methods: Vec<&Method<'a>> = vec![];

        // collect and group all children
        for child in &self.children {
            if let JavaDocableElement::Method(method) = child {
                methods.push(method);
            }
        }
        let methods_headline = t!("method_headline", nesting = prefix_hashes);

        //stringify all of the children with increased nesting levels
        let methods: Vec<String> = methods
            .iter()
            .map(|&child| child.render(level + 1))
            .collect();
        let methods = methods.join("\n");

        format!("\n\n{headline}\n\n{content}{methods_headline}\n\n{methods}")
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::parser::parse_string;

    use super::*;

    const NO_COMMENT: &str = "
// interface
interface Animal {
  public void animalSound(); // interface method (does not have a body)
  public void run(); // interface method (does not have a body)
}";

    const NO_JAVADOC: &str = "
/* interface */
interface Animal {
  public void animalSound(); // interface method (does not have a body)
  public void run(); // interface method (does not have a body)
}";

    const JAVADOC: &str = "
/** interface */
interface Animal {
  public void animalSound(); // interface method (does not have a body)
  public void run(); // interface method (does not have a body)
}";

    #[test]
    fn no_viable_comment() {
        let sourcecode = NO_COMMENT;
        let tree = parse_string(sourcecode).unwrap();
        let root = tree.root_node();
        let mut cursor = root.walk();
        let filecontext = FileContext::from_str(sourcecode).unwrap();

        let children: Vec<JavaDocableElement> = root
            .children(&mut cursor)
            .filter_map(|node| node_to_docable(node, &filecontext))
            .collect();
        assert_eq!(children.len(), 0);
    }

    #[test]
    fn no_javadoc_comment() {
        let sourcecode = NO_JAVADOC;
        let tree = parse_string(sourcecode).unwrap();
        let root = tree.root_node();
        let mut cursor = root.walk();
        let filecontext = FileContext::from_str(sourcecode).unwrap();

        let children: Vec<JavaDocableElement> = root
            .children(&mut cursor)
            .filter_map(|node| node_to_docable(node, &filecontext))
            .collect();
        assert_eq!(children.len(), 0);
    }
    #[test]
    fn javadoc_comment() {
        let sourcecode = JAVADOC;
        let tree = parse_string(sourcecode).unwrap();
        let root = tree.root_node();
        let mut cursor = root.walk();
        let filecontext = FileContext::from_str(sourcecode).unwrap();

        let children: Vec<JavaDocableElement> = root
            .children(&mut cursor)
            .filter_map(|node| node_to_docable(node, &filecontext))
            .collect();
        assert_eq!(children.len(), 1);
        let child = children.first().unwrap();
        match child {
            JavaDocableElement::Interface(child) => {
                assert_eq!(child.get_name(), "Animal");
                assert_eq!(child.children.len(), 0)
            }
            _ => panic!("Got something else than an interface????"),
        }
    }
}
