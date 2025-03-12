use anyhow::anyhow;
use std::path::{Path, PathBuf};

pub fn build_output_path(file: &PathBuf, out: &str) -> anyhow::Result<PathBuf> {
    let outdir = Path::new(out);
    match (file.is_absolute(), outdir.is_absolute()) {
        (true, true) => {
            // joining an absolute path into another path replaces the original completely
            let file = file.to_string_lossy();
            let file = &file[1..file.len()];
            let file = Path::new(file);
            let outdir = Path::new(out);
            let outdir = outdir.join(file);

            let outdir = outdir
                .parent()
                .ok_or_else(|| anyhow!("Failed to get parent"))?;

            let outdir = outdir.to_owned();
            return Ok(outdir);
        }
        (true, false) => {
            // joining an absolute path into another path replaces the original completely
            let file = file.to_string_lossy();
            let file = &file[1..file.len()];
            let file = Path::new(file);
            eprintln!("file {file:?}");
            let outdir = Path::new(out);
            eprintln!("outdir {outdir:?}");

            let outdir = outdir.join(file);
            eprintln!("outdir {outdir:?}");

            let outdir = outdir
                .parent()
                .ok_or_else(|| anyhow!("Failed to get parent"))?;
            eprintln!("outdir {outdir:?}");

            let outdir = outdir.to_owned();
            return Ok(outdir);
        },
        (false, true) => {
            let outdir = outdir.join(file.clone());
            let outdir = outdir
                .parent()
                .ok_or_else(|| anyhow!("Failed to get parent"))?;

            let outdir = outdir.to_owned();
            return Ok(outdir);
        }
        (false, false) => {
            let outdir = outdir.join(file.clone());
            let outdir = outdir
                .parent()
                .ok_or_else(|| anyhow!("Failed to get parent"))?;

            let outdir = outdir.to_owned();
            return Ok(outdir);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_simple_path() {
        let input = Path::new("src/java/main/superclass.java");
        let outdir = "tmp";

        let result = build_output_path(&input.to_path_buf(), outdir).unwrap();

        assert_eq!(result.to_string_lossy(), "tmp/src/java/main");
    }

    #[test]
    fn absolute_simple_path() {
        let input = Path::new("/data/src/java/main/superclass.java");
        let outdir = "/tmp";

        let result = build_output_path(&input.to_path_buf(), outdir).unwrap();

        assert_eq!(result.to_string_lossy(), "/tmp/data/src/java/main");
    }

    #[test]
    fn relative_nested_path() {
        let input = Path::new("src/java/main/superclass.java");
        let outdir = "tmp/out";

        let result = build_output_path(&input.to_path_buf(), outdir).unwrap();

        assert_eq!(result.to_string_lossy(), "tmp/out/src/java/main");
    }

    #[test]
    fn absolute_nested_path() {
        let input = Path::new("/data/src/java/main/superclass.java");
        let outdir = "/tmp/out";

        let result = build_output_path(&input.to_path_buf(), outdir).unwrap();

        assert_eq!(result.to_string_lossy(), "/tmp/out/data/src/java/main");
    }

    #[test]
    fn mixed_out_absolute() {
        let input = Path::new("src/java/main/superclass.java");
        let outdir = "/tmp";

        let result = build_output_path(&input.to_path_buf(), outdir).unwrap();

        assert_eq!(result.to_string_lossy(), "/tmp/src/java/main");
    }
    #[test]
    fn mixed_in_absolute() {
        let input = Path::new("/src/java/main/superclass.java");
        let outdir = "tmp";

        let result = build_output_path(&input.to_path_buf(), outdir).unwrap();

        assert_eq!(result.to_string_lossy(), "tmp/src/java/main");
    }
}
