use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the todo file.
    Add {
        /// The task description text.
        #[structopt()]
        task: String,
    },
    /// Remove an entry from the todo file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the todo file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Todo",
    about = "A command line to-do in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}