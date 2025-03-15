use javadoc2adoc_macros::*;

struct Node<'a> {
    thingy: &'a String,
}
struct BlockComment<'a> {
    thingy: &'a String,
}
struct FileContext {}

#[default_javadocable_fields]
struct Test<'a> {}

fn main() {}
