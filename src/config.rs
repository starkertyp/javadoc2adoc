use std::fmt::Display;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum Locale {
    En,
    De,
}

impl Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Locale::En => write!(f, "en"),
            Locale::De => write!(f, "de"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Config {
    /// Glob pattern for input files
    #[arg(short, long)]
    pub input: String,

    /// Output directory, will be created if not existing
    #[arg(short, long)]
    pub output: String,

    /// Localization to use for file generation
    #[arg(short, long, default_value_t = Locale::En)]
    pub locale: Locale,
}
