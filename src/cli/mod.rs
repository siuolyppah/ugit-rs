use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use crate::objects::manage;
use crate::objects::type_literal::ObjectTypeLiteral;

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
        /// Files to add content from. File globs (e.g. *.c) can be given to add all matching files.
        /// Also a leading directory name (e.g.  dir to add dir/file1 and dir/file2) can be given to
        /// update the index to match the current state of the directory as a whole (e.g. specifying
        /// dir will record not just a file dir/file1 modified in the working tree, a file dir/file2
        /// added to the working tree, but also a file dir/file3 removed from the working tree).
        /// Note that older versions of Git used to ignore removed files; use --no-all option if you
        /// want to add modified or new files but ignore removed ones.
        pathspec: Vec<PathBuf>,
    },

    /// Compute object ID and optionally create an object from a file
    HashObject {
        path: PathBuf,
        /// Specify the type of object to be created.
        #[arg(short = 't', long = "type", value_enum, default_value_t = ObjectTypeLiteral::Blob)]
        obj_type: ObjectTypeLiteral,
    },

    /// Provide content or type and size information for repository objects.
    CatFile {
        oid: String,
        #[arg(short = 't', long = "type", value_enum, default_value_t = ObjectTypeLiteral::Blob)]
        expected_type: ObjectTypeLiteral,
    },

    /// Create a tree object from the current index.
    WriteTree {},

    /// Reads tree information into the index.
    ReadTree { oid: String },
}

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init {}) => init::cmd_init(),
        Some(Commands::Add { pathspec }) => add::cmd_add(pathspec),
        Some(Commands::HashObject { path, obj_type }) => manage::cmd_hash_object(path, obj_type),
        Some(Commands::CatFile { oid, expected_type }) => manage::cmd_cat_file(oid, expected_type),
        Some(Commands::WriteTree {}) => manage::cmd_write_tree(Path::new(".")),
        Some(Commands::ReadTree { oid }) => manage::cmd_read_tree(oid),
        None => {
            // TODO: print help msg
        }
    }
}
