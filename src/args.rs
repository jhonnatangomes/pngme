use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode {
        file_path: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    Decode {
        file_path: PathBuf,
        chunk_type: String,
    },
    Remove {
        file_path: PathBuf,
        chunk_type: String,
    },
    Print {
        file_path: PathBuf,
    },
}
