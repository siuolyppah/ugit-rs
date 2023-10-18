use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::objects;

pub mod add;
pub mod init;

/// Simple program to learn more about how Git works on the inside.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create an empty Git repository or reinitialize an existing one
    ///
    /// # Example
    ///
    /// ```shell
    /// cargo run -- init
    /// ```
    Init {},

    /// Add file contents to the index
    ///
    /// # Example
    ///
    /// ```shell
    /// cargo run -- add abc.txt foo.txt
    /// ```
    Add {
        /// Files to add content from. Fileglobs (e.g. *.c) can be given to add all matching files.
        /// Also a leading directory name (e.g.  dir to add dir/file1 and dir/file2) can be given to
        /// update the index to match the current state of the directory as a whole (e.g. specifying
        /// dir will record not just a file dir/file1 modified in the working tree, a file dir/file2
        /// added to the working tree, but also a file dir/file3 removed from the working tree).
        /// Note that older versions of Git used to ignore removed files; use --no-all option if you
        /// want to add modified or new files but ignore removed ones.
        pathspec: Vec<PathBuf>,
    },

    /// Compute object ID and optionally create an object from a file
    ///
    /// # Example
    ///
    /// ```shell
    /// cargo r -- hash-object foo.txt
    /// ```
    HashObject { file: PathBuf },

    /// Provide content or type and size information for repository objects
    ///
    /// # Example
    ///
    /// ```shell
    /// cargo r -- cat-file c254a8e49ef377fe15554aa37ad91bb97264e50f
    /// ```
    CatFile { oid: String },
}

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init {}) => init::cmd_init(),
        Some(Commands::Add { pathspec }) => add::cmd_add(pathspec),
        Some(Commands::HashObject { file }) => objects::cmd_hash_object(file),
        Some(Commands::CatFile { oid }) => objects::cmd_cat_file(oid),
        None => {
            // TODO: print help msg
        }
    }
}
