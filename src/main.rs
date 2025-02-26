use std::str::FromStr;

use classdoc::Class;
use tracing::trace;

mod classdoc;

const TESTCLASS: &str = include_str!("test.java");

fn main() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
    let classdoc = Class::from_sourcecode(TESTCLASS, 0)?;

    trace!("Got {classdoc:?}");

    println!("{classdoc}");

    Ok(())
}
 
