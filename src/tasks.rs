use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};

use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Consume the file's contents as a vector of tasks.
    let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Rewind the file after reading from it.
    file.seek(SeekFrom::Start(0))?;

    // Write the modified task list back into the file.
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Tesult<()> {
    let file = OpenOption::new()
        .read(true)
        .write(true)
        .open(journal_path)?;
    
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    
    if task_position == 0 || task_position > task.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    task.remove(task_position - 1);

    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;

    serde_json::to_writer(file, &tasks)?;
    Ok(())
}