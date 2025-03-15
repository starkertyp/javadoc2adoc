use javadoc2adoc_macros::{default_javadocable_fields, DefaultJavaDocable};
use javadoc2adoc_types::DefaultJavaDocable;
use rust_i18n::t;
use tracing::{debug, instrument};
use tree_sitter::Node;

use crate::javadoc::{constructor::Constructor, field::Field, method::Method, prefix_hashes};

use super::{
    comment::find_block_comment, node_to_docable, FileContext, JavaDocable, JavaDocableElement,
};

#[default_javadocable_fields]
#[derive(Debug, DefaultJavaDocable)]
pub struct Class<'a> {
    children: Vec<JavaDocableElement<'a>>,
}

impl<'a> JavaDocable<'a> for Class<'a> {
    #[instrument(skip_all)]
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
            debug!("Found a class but no block comment, skipping");
            None
        }
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

        let mut fields: Vec<&Field<'a>> = vec![];
        let mut methods: Vec<&Method<'a>> = vec![];
        let mut classes: Vec<&Class<'a>> = vec![];
        let mut constructors: Vec<&Constructor<'a>> = vec![];

        // collect and group all children
        for child in &self.children {
            match child {
                JavaDocableElement::Class(class) => {
                    classes.push(class);
                }
                JavaDocableElement::Field(field) => {
                    fields.push(field);
                }
                JavaDocableElement::Method(method) => {
                    methods.push(method);
                }
                JavaDocableElement::Constructor(constructor) => {
                    constructors.push(constructor);
                }
                JavaDocableElement::Interface(_) => (),
            }
        }
        let fields_headline = t!("field_headline", nesting = prefix_hashes);
        let methods_headline = t!("method_headline", nesting = prefix_hashes);
        let classes_headline = t!("class_headline", nesting = prefix_hashes);
        let constructors_headline = t!("constructor_headline", nesting = prefix_hashes);

        //stringify all of the children with increased nesting levels
        let fields: Vec<String> = fields
            .iter()
            .map(|&child| child.render(level + 1))
            .collect();
        let fields = fields.join("\n");
        let methods: Vec<String> = methods
            .iter()
            .map(|&child| child.render(level + 1))
            .collect();
        let methods = methods.join("\n");
        let classes: Vec<String> = classes
            .iter()
            .map(|&child| child.render(level + 1))
            .collect();
        let classes = classes.join("\n");
        let constructors: Vec<String> = constructors
            .iter()
            .map(|&child| child.render(level + 1))
            .collect();
        let constructors = constructors.join("\n");

        format!("\n\n{headline}\n\n{content}{constructors_headline}\n\n{constructors}{fields_headline}\n\n{fields}{methods_headline}\n\n{methods}{classes_headline}\n\n{classes}")
    }
}
