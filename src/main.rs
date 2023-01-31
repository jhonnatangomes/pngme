use args::parse_args;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    match parse_args() {
        Ok(r) => println!("{r}"),
        Err(e) => eprintln!("{e}"),
    }
    Ok(())
}
