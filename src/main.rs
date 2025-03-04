use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use classdoc::Class;
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

async fn doc_from_file(path: &PathBuf) -> anyhow::Result<Option<Class>> {
    let content = read_to_string(path).await?;
    let class = Class::from_sourcecode(&content, 0)?;
    Ok(class)
}

#[apply(main!)]
async fn main(ex: &Executor<'_>) -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let mut args = env::args().skip(1);
    let glob_in = args
        .next()
        .ok_or_else(|| anyhow!("expected a glob pattern as the first parameter"))?;
    debug!("glob pattern: {glob_in:?}");
    let outdir = args
        .next()
        .ok_or_else(|| anyhow!("expected an out dir as the second parameter"))?;

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
                        if let Some(classdoc) = classdoc {
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
                            write(outpath, format!("{classdoc}")).await.unwrap()
                        } else {
                            debug!("Skipping {entry:?} as it doesn't contain a class");
                        }
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
