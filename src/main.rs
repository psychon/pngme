// https://www.reddit.com/r/rust/comments/gzehpz/pngme_an_intermediate_rust_project/
// https://picklenerd.github.io/pngme_book/

use structopt::StructOpt;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod crc;
mod png;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let opt = args::Command::from_args();
    println!("{:?}", opt);
    Ok(())
}
