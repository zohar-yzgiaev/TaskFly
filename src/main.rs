use std::str::FromStr;
use structopt::StructOpt;
use anyhow::{Result, Context};
use unicode_segmentation::UnicodeSegmentation;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub enum UrgencyLevel {
    #[serde(rename = "🚀")]
    BlastOff,
    #[serde(rename = "✈️")]
    JetStream,
    #[serde(rename = "🚲")]
    DailyCruise,
    #[serde(rename = "🐢")]
    TurtlePace,
}

impl FromStr for UrgencyLevel {
    type Err = &'static str;

    fn from_str(level: &str) -> Result<Self, Self::Err> {
        let emoji = level.graphemes(true).next().ok_or("Invalid emoji representation")?;

        match emoji {
            "🚀" => Ok(UrgencyLevel::BlastOff),
            "✈️" => Ok(UrgencyLevel::JetStream),
            "🚲" => Ok(UrgencyLevel::DailyCruise),
            "🐢" => Ok(UrgencyLevel::TurtlePace),
            _ => Err("Unknown urgency level"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    level: UrgencyLevel,
    command: String,
}

impl Task {
    fn new(level: UrgencyLevel, command: String) -> Self {
        Task { level, command }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn add_task(&mut self, level: UrgencyLevel, command: String) {
        let task = Task::new(level, command);
        self.tasks.push(task);
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
        } else {
            for (index, task) in self.tasks.iter().enumerate() {
                println!("Task {}: Command: '{}', Urgency Level: {:?}", index + 1, task.command, task.level);
            }
        }
    }

    fn save_tasks(&self, filename: &str) -> Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, self).context("Failed to save tasks")?;
        Ok(())
    }

    fn load_tasks(filename: &str) -> Result<Self> {
        let mut file = OpenOptions::new().read(true).open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;        

        let task_manager: TaskManager = serde_json::from_str(&contents)
            .context("Failed to load tasks")?;

        Ok(task_manager)
    }

    fn remove_task(&mut self, command: &str) -> Result<(), String> {
        let index = self.tasks.iter().position(|task| task.command == command);
        if let Some(index) = index {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(format!("Task with command '{}' not found", command))
        }
    }
    
    fn edit_task(&mut self, command: &str, level: UrgencyLevel) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.command == command) {
            task.level = level;
            Ok(())
        } else {
            Err(format!("Task with command '{}' not found", command))
        }
    }    
}

#[derive(StructOpt, Debug)]
#[structopt(name = "taskfly")]
pub enum TaskFly {
    #[structopt(name = "add")]
    Add {
        #[structopt(name = "UrgencyLevel")]
        level: UrgencyLevel,
        #[structopt(name = "Command")]
        command: String,
    },
    #[structopt(name = "list")]
    List,
    #[structopt(name = "Remove")]
    Remove {
        #[structopt(name = "Command")]
        command: String,
    },
    #[structopt(name = "edit")]
    Edit {
        #[structopt(name = "Command")]
        command: String,
        #[structopt(name = "UrgencyLevel")]
        level: UrgencyLevel,
    },
}

fn main() -> Result<()> {
    let mut task_manager = TaskManager::load_tasks("tasks.json").unwrap_or_default();
    let opt = TaskFly::from_args();

    match opt {
        TaskFly::Add { level, command } => {
            task_manager.add_task(level, command);
            println!("Task added successfully!");
        }
        TaskFly::List => {
            task_manager.list_tasks();
        }
        TaskFly::Remove { command } => {
            match task_manager.remove_task(&command) {
                Ok(()) => println!("Task removed successfully!"),
                Err(err) => eprintln!("Failed to remove task: {}", err),
            }
        }
        TaskFly::Edit { command, level } => {
            match task_manager.edit_task(&command, level) {
                Ok(()) => println!("Task edited successfully!"),
                Err(err) => eprintln!("Failed to edit task: {}", err),
            }
        }
    }

    task_manager.save_tasks("tasks.json")?;
    Ok(())
}
