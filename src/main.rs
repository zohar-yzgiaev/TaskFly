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
            // TODO: Handle adding task
            println!("Adding task with command '{}' and urgency level '{:?}'", command, level);
        }
        TaskFly::List => {
            // TODO: Handle listing tasks
            println!("Listing tasks");
        }
        TaskFly::Remove { command } => {
            // TODO: Handle removing task
            println!("Removing task with command '{}'", command);
        }
        TaskFly::Edit { command, level } => {
            // TODO: Handle editing task
            println!("Editing task with command '{}' to urgency level '{:?}'", command, level);
        }
    }

    Ok(())
}