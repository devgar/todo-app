use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()>{
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_path,
    } = CommandLineArgs::from_args();

    let journal_path = journal_path
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    match action {
        Add { text } => tasks::add_task(journal_path, Task::new(text)),
        List => tasks::list_tasks(journal_path),
        Done { position } => tasks::complete_task(journal_path, position),
    }?;
    Ok(())
}
