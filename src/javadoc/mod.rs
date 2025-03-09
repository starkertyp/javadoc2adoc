pub mod class;
pub mod comment;
pub mod constructor;
pub mod field;
pub mod interface;
pub mod method;

use std::str::FromStr;

use class::Class;
use comment::BlockComment;
use constructor::Constructor;
use field::Field;
use interface::Interface;
use method::Method;
use tracing::{debug, trace};
use tree_sitter::{Node, Range};

#[derive(Debug)]
pub struct FileContext(String);

impl FileContext {
    pub fn source_for_range(&self, range: &Range) -> &str {
        let sourcecode = &self.0;
        let sourcecode = &sourcecode[range.start_byte..range.end_byte];
        sourcecode
    }
}

impl FromStr for FileContext {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

pub trait JavaDocable<'a> {
    fn new(ctx: &'a FileContext, node: Node<'a>) -> Option<Self>
    where
        Self: Sized;
    fn get_node(&self) -> Node<'_>;
    fn get_context(&self) -> &'a FileContext;
    fn get_comment(&self) -> &'a BlockComment;
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
            match class {
                Some(class) => Some(JavaDocableElement::Class(class)),
                None => None,
            }
        }
        "method_declaration" => {
            debug!("Found a method declaration");
            let method = Method::new(ctx, node);
            match method {
                Some(method) => Some(JavaDocableElement::Method(method)),
                None => None,
            }
        }
        "field_declaration" => {
            debug!("Found a field declaration");
            let field = Field::new(ctx, node);
            match field {
                Some(field) => Some(JavaDocableElement::Field(field)),
                None => None,
            }
        }
        "constructor_declaration" => {
            debug!("Found a constructor declaration");
            let constructor = Constructor::new(ctx, node);
            match constructor {
                Some(constructor) => Some(JavaDocableElement::Constructor(constructor)),
                None => None,
            }
        }
        "interface_declaration" => {
            debug!("Found a interface declaration");
            let interface = Interface::new(ctx, node);
            match interface {
                Some(interface) => Some(JavaDocableElement::Interface(interface)),
                None => None,
            }
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
    use tree_sitter::Point;

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
