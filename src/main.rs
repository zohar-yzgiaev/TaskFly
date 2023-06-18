use std::str::FromStr;
use structopt::StructOpt;
use anyhow::Result;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub enum UrgencyLevel {
    BlastOff,
    JetStream,
    DailyCruise,
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

#[derive(StructOpt, Debug)]
#[structopt(name = "taskfly")]
pub enum TaskFly {
    #[structopt(name = "Add")]
    Add {
        #[structopt(name = "UrgencyLevel")]
        level: UrgencyLevel,
        #[structopt(name = "Command")]
        command: String,
    },
    #[structopt(name = "List")]
    List,
    #[structopt(name = "Remove")]
    Remove {
        #[structopt(name = "Command")]
        command: String,
    },
    #[structopt(name = "Edit")]
    Edit {
        #[structopt(name = "Command")]
        command: String,
        #[structopt(name = "UrgencyLevel")]
        level: UrgencyLevel,
    },
}

fn main() -> Result<()> {
    let opt = TaskFly::from_args();

    match opt {
        TaskFly::Add { level, command } => {
            add_task(level, command)?;
        }
        TaskFly::List => {
            list_tasks()?;
        }
        TaskFly::Remove { command } => {
            remove_task(command)?;
        }
        TaskFly::Edit { command, level } => {
            edit_task(command, level)?;
        }
    }

    Ok(())
}

fn add_task(level: UrgencyLevel, command: String) -> Result<()> {
    // TODO: Implement the logic to add a task
    println!("Adding task with command '{}' and urgency level '{:?}'", command, level);

    Ok(())
}

fn list_tasks() -> Result<()> {
    // TODO: Implement the logic to list tasks
    println!("Listing tasks");

    Ok(())
}

fn remove_task(command: String) -> Result<()> {
    // TODO: Implement the logic to remove a task
    println!("Removing task with command '{}'", command);

    Ok(())
}

fn edit_task(command: String, level: UrgencyLevel) -> Result<()> {
    // TODO: Implement the logic to edit a task
    println!("Editing task with command '{}' to urgency level '{:?}'", command, level);

    Ok(())
}