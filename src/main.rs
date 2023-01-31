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

fn ask_complete(journal_path:& PathBuf) -> std::io::Result<usize> {
    dialog::ask(&tasks::get_tasks(journal_path)?)
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
        Done { task_position } => {
            let task_position = task_position
                .unwrap_or_else(|| ask_complete(&journal_path)
                .unwrap());
            tasks::complete_task(journal_path, task_position)
        },
    }?;
    Ok(())
}
