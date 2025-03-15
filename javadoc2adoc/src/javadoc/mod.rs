pub mod class;
pub mod comment;
pub mod constructor;
pub mod field;
pub mod interface;
pub mod method;

use class::Class;
use constructor::Constructor;
use field::Field;
use interface::Interface;
use javadoc2adoc_types::FileContext;
use method::Method;
use tracing::{debug, trace};
use tree_sitter::Node;

pub trait JavaDocable<'a>: javadoc2adoc_types::DefaultJavaDocable<'a> {
    fn new(ctx: &'a FileContext, node: Node<'a>) -> Option<Self>
    where
        Self: Sized;
    fn get_name(&self) -> String;
    fn render(&'a self, level: u8) -> String {
        let prefix_hashes = prefix_hashes(level);
        let name = self.get_name();
        let headline = format!("{prefix_hashes}= {name}");
        let content = self.get_comment();
        let content = format!("{content}");
        format!("\n\n{headline}\n\n{content}")
    }
}

#[derive(Debug)]
pub enum JavaDocableElement<'a> {
    Class(Class<'a>),
    Field(Field<'a>),
    Method(Method<'a>),
    Constructor(Constructor<'a>),
    Interface(Interface<'a>),
}

pub fn node_to_docable<'a>(node: Node<'a>, ctx: &'a FileContext) -> Option<JavaDocableElement<'a>> {
    let name = node.grammar_name();
    trace!("Handling a {name} node");
    match name {
        "class_declaration" => {
            debug!("Found a class declaration");
            let class = Class::new(ctx, node);
            class.map(JavaDocableElement::Class)
        }
        "method_declaration" => {
            debug!("Found a method declaration");
            let method = Method::new(ctx, node);
            method.map(JavaDocableElement::Method)
        }
        "field_declaration" => {
            debug!("Found a field declaration");
            let field = Field::new(ctx, node);
            field.map(JavaDocableElement::Field)
        }
        "constructor_declaration" => {
            debug!("Found a constructor declaration");
            let constructor = Constructor::new(ctx, node);
            constructor.map(JavaDocableElement::Constructor)
        }
        "interface_declaration" => {
            debug!("Found a interface declaration");
            let interface = Interface::new(ctx, node);
            interface.map(JavaDocableElement::Interface)
        }
        _ => None,
    }
}

pub fn prefix_hashes(level: u8) -> String {
    let level: usize = level.into();
    let prefix_hashes = vec!["="; level].join("");
    format!("={}", prefix_hashes)
}

#[cfg(test)]
mod filecontext_tests {
    use std::str::FromStr;

    use javadoc2adoc_types::FileContext;
    use tree_sitter::{Point, Range};

    use super::*;

    const INPUT: &str =
        "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod";

    #[test]
    fn from_str() {
        let ctx = FileContext::from_str(INPUT);
        assert!(ctx.is_ok());
    }

    #[test]
    fn source_for_range() {
        let ctx = FileContext::from_str(INPUT).unwrap();
        let p = Point::default();
        let range = Range {
            start_byte: 2,
            end_byte: 7,
            start_point: p,
            end_point: p,
        };
        assert_eq!(ctx.source_for_range(&range), "rem i");
    }
}
