use clap::Parser;
use config::Config;
use rust_i18n::{i18n, set_locale};
use std::path::{Path, PathBuf};

use classdoc::from_sourcecode;
use futures::future::join_all;
use glob::glob;
use macro_rules_attribute::apply;
use smol::{
    fs::{read_to_string, write, DirBuilder},
    Executor, Task,
};
use smol_macros::main;
use tracing::{debug, info, trace};

mod classdoc;
mod javadoc;
mod config;

i18n!();

async fn doc_from_file(path: &PathBuf) -> anyhow::Result<String> {
    let content = read_to_string(path).await?;
    let rendered = from_sourcecode(&content)?;
    Ok(rendered)
}

#[apply(main!)]
async fn main(ex: &Executor<'_>) -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cfg = Config::parse();

    let glob_in = cfg.input;
    debug!("glob pattern: {glob_in:?}");
    let outdir = cfg.output;
    debug!("out dir: {outdir}");
    let locale = cfg.locale;
    debug!("locale: {locale}");
    set_locale(&locale.to_string());
    
    let mut tasks: Vec<Task<()>> = vec![];
    for entry in glob(&glob_in)? {
        let entry = entry?;
        info!("Trying to handle file {entry:?}");
        if entry.is_file() {
            trace!("Is a file");
            if let Some(extension) = entry.extension() {
                trace!("Has an extension");
                if extension == "java" {
                    debug!("Found java file at {entry:?}");
                    let outdir = outdir.clone(); // clone to work around move
                    let task = ex.spawn(async move {
                        let classdoc = doc_from_file(&entry).await.unwrap();
                        trace!("Got {classdoc:?}");
                        let outdir = Path::new(&outdir);
                        let filename = entry.file_name().unwrap();
                        let filename = filename.to_string_lossy().replace(".java", ".adoc");
                        let outdir = outdir.join(entry.clone());
                        let outdir = outdir.parent().unwrap();
                        trace!("Built outdir {outdir:?}");
                        DirBuilder::new()
                            .recursive(true)
                            .create(outdir)
                            .await
                            .unwrap();
                        trace!("Outdir {outdir:?} created");
                        let outpath = outdir.join(filename);
                        debug!("Writing to {outpath:?}");
                        write(outpath, classdoc).await.unwrap()
                    });
                    tasks.push(task);
                }
            }
        }
    }
    join_all(tasks).await;

    // ex.spawn(async {
    //     println!("Hello world!");
    // })
    // .await;

    // tracing_subscriber::fmt::init();
    // let classdoc = Class::from_sourcecode(TESTCLASS, 0)?;

    Ok(())
}
