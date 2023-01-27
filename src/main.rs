use structopt::StructOpt;

mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn main() {
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_path,
    } = CommandLineArgs::from_args();

    let journal_path = journal_path.expect("Failed to find journal file");

    match action {
        Add { text } => tasks::add_task(journal_path, Task::new(text)),
        List => tasks::list_tasks(journal_path),
        Done { position } => tasks::complete_task(journal_path, position),
    }
    .expect("Failed to perform action")
}
