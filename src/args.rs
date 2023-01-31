use crate::{
    commands::{print, remove},
    Result,
};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::commands::{decode, encode};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Encode a secret message in the given chunk
    Encode {
        file_path: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    /// Decode the secret message contained in the given chunk
    Decode {
        file_path: PathBuf,
        chunk_type: String,
    },
    /// Remove the secret message contained in the given chunk
    Remove {
        file_path: PathBuf,
        chunk_type: String,
    },
    /// Prints the chunk types that have hidden messages
    Print { file_path: PathBuf },
}

pub fn parse_args() -> Result<String> {
    let args = Args::parse();
    match &args.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => encode(file_path, chunk_type, message, output_file),
        Commands::Decode {
            file_path,
            chunk_type,
        } => decode(file_path, chunk_type),
        Commands::Remove {
            file_path,
            chunk_type,
        } => remove(file_path, chunk_type),
        Commands::Print { file_path } => print(file_path),
    }
}
