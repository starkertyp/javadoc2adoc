pub mod comment;
pub mod field;
pub mod class;

use std::{fmt::Display, str::FromStr};

use class::Class;
use field::Field;
use tracing::debug;
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

impl FromStr for FileContext{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

pub trait JavaDocable<'a> {
    fn new(ctx: &'a FileContext, node: Node<'a>) -> Option<Self>
    where
        Self: Sized;
    fn has_children(&self) -> bool;
    fn get_node(&self) -> Node<'_>;
    fn get_context(&self) -> &'a FileContext;
    fn get_name(&self) -> String;
    fn render(&'a self) {
        todo!()
    }
}

pub fn node_to_docable<'a>(node: Node<'a>, ctx: &'a FileContext ) -> Option<Box<dyn JavaDocable<'a> + 'a>> {
    let name = node.grammar_name();
    match name {
        "class_declaration" => {
            debug!("Found a class declaration");
            let class = Class::new(ctx, node);
            match class {
                Some(class) => Some(Box::new(class)),
                None => None
            }
        }
        "method_declaration" => todo!(),
        "field_declaration" => {
            debug!("Found a field declaration");
            let field = Field::new(ctx, node);
            match field {
                Some(field) => Some(Box::new(field)),
                None => None
            }
        },
        "constructor_declaration" => todo!(),
        _ => None
    }
    
}
