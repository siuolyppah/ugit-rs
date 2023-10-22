#[macro_use]
mod unexpected;

mod cli;
mod crypto;
mod fs_tools;
mod objects;

fn main() {
    cli::run();
}
