use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

mod cli;
mod tasks;
mod dialog;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn ask_complete(journal_path: PathBuf) -> std::io::Result<()> {
    let journal_clone = journal_path.clone();
    let items = tasks::get_tasks(journal_path)?;
    let task_position = dialog::ask(&items)?;
    tasks::complete_task(journal_clone, task_position)
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
        Done { task_position } => match task_position {
            Some(task_position) => tasks::complete_task(journal_path, task_position),
            None => ask_complete(journal_path),
        },
    }?;
    Ok(())
}
