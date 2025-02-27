use std::{fmt, str::FromStr};

use anyhow::{anyhow, bail};
use tracing::{debug, instrument, trace, warn, Instrument};
use tree_sitter::{Node, Parser, Tree, TreeCursor};

#[derive(Debug)]
struct Field {
    name: String,
    comment: String,
    level: u32,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level: usize = self.level.try_into().unwrap();
        let prefix_hashes = vec!["="; level].join("");
        writeln!(f, "==={} {}", prefix_hashes, self.name)?;
        writeln!(f, "{}", self.comment)
    }
}

impl Field {
    #[instrument(skip_all)]
    pub fn from_node<'a>(
        node: &Node<'a>,
        sourcecode: &'a str,
        comment: String,
        level: u32,
    ) -> anyhow::Result<Self> {
        let declarator = node
            .child_by_field_name("declarator")
            .ok_or_else(|| anyhow!("Expected a field to have a declarator"))?;
        let node_type = node
            .child_by_field_name("type")
            .ok_or_else(|| anyhow!("Expected a field to have a type"))?;
        let declarator = get_string_of_node(&declarator, &sourcecode);
        let thing_type = get_string_of_node(&node_type, &sourcecode);
        let name = format!("{thing_type} {declarator}");
        Ok(Self {
            name: name.to_string(),
            comment,
            level,
        })
    }
}

#[derive(Debug)]
struct Method {
    name: String,
    comment: String,
    level: u32,
}

impl Method {
    #[instrument(skip_all)]
    pub fn from_node<'a>(
        node: &Node<'a>,
        sourcecode: &'a str,
        comment: String,
        level: u32,
    ) -> anyhow::Result<Self> {
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("Expected a method to have a name"))?;
        let node_type = node
            .child_by_field_name("type")
            .ok_or_else(|| anyhow!("Expected a method to have a type"))?;
        let params = node
            .child_by_field_name("parameters")
            .ok_or_else(|| anyhow!("Expected a method to have a parameter declaration"))?;

        let name = get_string_of_node(&name, &sourcecode);
        let node_type = get_string_of_node(&node_type, &sourcecode);
        let params = get_string_of_node(&params, &sourcecode);
        let name = format!("{node_type} {name} {params}");

        Ok(Self {
            name: name.to_string(),
            comment,
            level,
        })
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level: usize = self.level.try_into().unwrap();
        let prefix_hashes = vec!["="; level].join("");
        writeln!(f, "==={} {}", prefix_hashes, self.name)?;
        writeln!(f, "{}", self.comment)
    }
}

#[derive(Debug)]
struct Constructor {
    name: String,
    comment: String,
    level: u32,
}

impl Constructor {
    #[instrument(skip_all)]
    pub fn from_node<'a>(
        node: &Node<'a>,
        sourcecode: &'a str,
        comment: String,
        level: u32,
    ) -> anyhow::Result<Self> {
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("Expected a constructor to have a name"))?;
        let params = node
            .child_by_field_name("parameters")
            .ok_or_else(|| anyhow!("Expected a constructor to have a parameter declaration"))?;

        let name = get_string_of_node(&name, &sourcecode);
        let params = get_string_of_node(&params, &sourcecode);
        let name = format!("{name} {params}");

        Ok(Self {
            name: name.to_string(),
            comment,
            level,
        })
    }
}

impl fmt::Display for Constructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level: usize = self.level.try_into().unwrap();
        let prefix_hashes = vec!["="; level].join("");
        writeln!(f, "==={} {}", prefix_hashes, self.name)?;
        writeln!(f, "{}", self.comment)
    }
}

#[derive(Debug, Default)]
pub struct Class {
    level: u32,
    name: String,
    class_comment: String,
    children: Vec<Class>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    constructors: Vec<Constructor>,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level: usize = self.level.try_into().unwrap();
        let prefix_hashes = vec!["="; level].join("");
        write!(f, "={} {}", prefix_hashes, self.name)?;
        write!(f, "{}", self.class_comment)?;

        if self.constructors.len() > 0 {
            writeln!(f, "=={} Konstruktoren", prefix_hashes)?
        }
        for constructor in &self.constructors {
            write!(f, "{}", constructor)?;
        }

        if self.fields.len() > 0 {
            writeln!(f, "=={} Felder", prefix_hashes)?
        }
        for field in &self.fields {
            write!(f, "{}", field)?;
        }

        if self.methods.len() > 0 {
            writeln!(f, "=={} Methoden", prefix_hashes)?
        }
        for method in &self.methods {
            write!(f, "{}", method)?;
        }

        if self.children.len() > 0 {
            writeln!(f, "=={} Subklassen", prefix_hashes)?
        }
        for child in &self.children {
            write!(f, "{}", child)?;
        }
        Ok(())
    }
}

impl Class {
    #[instrument(skip_all)]
    pub fn from_sourcecode(sourcecode: &str, level: u32) -> Result<Option<Self>, anyhow::Error> {
        if level > 0 {
            assert_eq!(level % 2, 0);
        }
        let tree = parse_string(sourcecode)?;
        debug!("Getting root node first");
        let root = tree.root_node();
        let mut cursor = root.walk();
        let mut classdoc = Self {
            level,
            ..Default::default()
        };

        if let Some(class_comment) = get_class_comment(&root, &mut cursor, sourcecode) {
            classdoc.class_comment = class_comment;
        }

        debug!("Looking for the root class");
        let root_classes = find_classes(&root, &mut cursor);
        if root_classes.len() == 0 {
            debug!("no class found, returning early");
            return Ok(None);
        }
        if root_classes.len() > 1 {
            warn!(
                "Expected exactly one root class, got {} and don't know how to handle that",
                root_classes.len()
            );
            return Ok(None);
        }
        let root = root_classes.get(0).unwrap();
        trace!("Root class is {root:?}");
        let name = root
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("Expected a class to have a name"))?;
        trace!("Found a name node: {name:?}");
        let name = get_string_of_node(&name, sourcecode);
        trace!("Found name: {name:?}");
        classdoc.name = name.to_string();
        let root = root
            .child_by_field_name("body")
            .ok_or_else(|| anyhow!("Expected a class to have a body"))?;
        trace!("Found a body node: {root:?}");

        let comments: Vec<Node<'_>> = root
            .children(&mut cursor)
            .filter(|child| is_javadoc_comment(child, sourcecode))
            .collect();
        if comments.len() > 0 {
            trace!(
                "Found {} javadoc comments inside the class!",
                comments.len()
            );
        }
        for comment_node in comments {
            let comment = get_string_of_node(&comment_node, sourcecode);
            let comment = javadoc_to_adoc(comment);
            let thing = comment_node
                .next_sibling()
                .ok_or_else(|| anyhow!("Expected a javadoc comment to be for something"))?;
            match thing.grammar_name() {
                "field_declaration" => {
                    debug!("Found a field declaration");
                    let field = Field::from_node(&thing, sourcecode, comment, level)?;
                    classdoc.fields.push(field);
                }
                "method_declaration" => {
                    debug!("Found a method declaration");
                    let method = Method::from_node(&thing, sourcecode, comment, level)?;
                    classdoc.methods.push(method);
                }
                "class_declaration" => {
                    debug!("Found a class");
                    // need to parse the comment as well!
                    let comment_range = comment_node.range();
                    let class_range = thing.range();
                    let sourcecode = &sourcecode[comment_range.start_byte..class_range.end_byte];
                    let class = Class::from_sourcecode(sourcecode, level + 2)?;
                    if let Some(class) = class {
                        classdoc.children.push(class);
                    }
                }
                "constructor_declaration" => {
                    debug!("Found a constructor declaration");
                    let constructor = Constructor::from_node(&thing, sourcecode, comment, level)?;
                    classdoc.constructors.push(constructor);
                }
                _ => debug!("Got {}, which is unsupported", thing.grammar_name()),
            }
        }

        Ok(Some(classdoc))
    }
}

#[instrument(skip_all)]
fn find_classes<'a>(node: &Node<'a>, cursor: &mut TreeCursor<'a>) -> Vec<Node<'a>> {
    node.children(cursor)
        .filter(|node| node.grammar_name() == "class_declaration")
        .collect()
}

#[instrument(skip_all)]
fn get_class_comment<'a>(
    root: &Node<'a>,
    cursor: &mut TreeCursor<'a>,
    sourcecode: &str,
) -> Option<String> {
    debug!("Looking for class comment");
    let comments: Vec<Node<'_>> = root
        .children(cursor)
        .filter(|child| is_javadoc_comment(child, sourcecode))
        .collect();
    if comments.len() > 0 {
        debug!("Found at least one javadoc at the root level! Using the first one");
        let class_comment = comments.first().unwrap();
        let class_comment = get_string_of_node(class_comment, sourcecode);
        trace!("comment is: {class_comment:?}");
        let class_comment = javadoc_to_adoc(class_comment);
        trace!("comment after conversion: {class_comment:?}");
        return Some(class_comment);
    }
    None
}

#[instrument(skip_all)]
fn get_string_of_node<'a>(node: &Node<'a>, sourcecode: &'a str) -> &'a str {
    let range = node.range();
    trace!("range of node: {range:?}");
    let content = &sourcecode[range.start_byte..range.end_byte];
    content
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

#[instrument(skip_all)]
fn is_javadoc_comment(node: &Node<'_>, source: &str) -> bool {
    trace!("Looking at {node:?}");
    if node.grammar_name() == "block_comment" {
        trace!("Node is a block comment");
        let range = node.range();
        // look at the first three bytes, they should be /**
        let start = &source[range.start_byte..range.start_byte + 3];
        trace!("Node is starting with: {start:?}");
        return start == "/**";
    }
    false
}

#[instrument(skip_all)]
fn javadoc_to_adoc(source: &str) -> String {
    let lines: Vec<String> = source
        .lines()
        .map(|line| {
            let line = line.trim();
            if line.starts_with("/**") {
                trace!("Stripping '/**' from {line:?}");
                return line.strip_prefix("/**").unwrap().trim_start();
            } else if line.starts_with("*/") {
                trace!("Stripping '*/' from {line:?}");
                return line.strip_prefix("*/").unwrap().trim_start();
            } else if line.starts_with("*") {
                trace!("Stripping '*' from {line:?}");
                return line.strip_prefix("*").unwrap().trim_start();
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
                    return format!("{head}::{tail}");
                } else {
                    return line.to_string();
                }
            } else {
                return line.to_string();
            }
        })
        .collect();

    lines.join("\n")
}
