# 🚀 TaskFly - The Fun Job Scheduler Daemon

Experience a whole new way of scheduling tasks with TaskFly. Representing tasks' urgency with emojis, TaskFly not only makes job scheduling fun but also intuitive and user-friendly. Now with an added ability to schedule tasks over HTTP, TaskFly broadens its accessibility to other languages and systems. 

## 🤔 What is TaskFly?

TaskFly is a daemon designed to schedule tasks according to their urgency level, symbolized by one of four emojis. It simplifies the task scheduling experience while ensuring tasks are carried out at the right time based on their urgency. With an HTTP interface for easy integration with your existing services, TaskFly also provides advanced control and monitoring capabilities for your scheduled tasks.

Here are the available urgency levels:

- 🚀 Blast Off: Super urgent tasks that need to be executed within the next 30 minutes.
- ✈️ Jet Stream: Urgent tasks that need to be executed within the next 2 hours.
- 🚲 Daily Cruise: Tasks that need to be executed by the end of the day.
- 🐢 Turtle Pace: Tasks that can wait to be executed sometime this week.

With its layered architecture, TaskFly is more than just a scheduler - it's a powerful, flexible daemon with a user-friendly interface that handles job dependencies, manages execution environments, offers direct feedback on job statuses, and provides a centralized platform for task monitoring.

## 🏛️ Architecture

TaskFly follows a three-tiered architecture:

1. **Daemon Layer**: The overarching layer that manages the HTTP Service and Task Scheduler layers. 
2. **HTTP Service Layer**: Responsible for handling incoming HTTP requests and forwarding them to the Task Scheduler.
3. **Task Scheduler Layer**: Schedules and executes tasks according to their urgency level.


## 🔧 Installation

...[Insert installation instructions here]

## 🎮 Usage

Adding a task is as simple as this:

```bash
$ taskfly add "Blast Off 🚀" "/path/to/your/script.sh"
```

And you can list your tasks with:

```bash
$ taskfly list
```

To remove a task, use:

```bash
$ taskfly remove "Your task description here"
```

Want to change the urgency of a task? Here you go:
```bash
$ taskfly edit "Your task description here" "Turtle Pace 🐢"
```

And, of course, you can always ask TaskFly for help:

```bash
$ taskfly --help
```

## 💪 How to Contribute
We welcome all contributors to TaskFly! Whether it's reporting a bug, proposing a new feature, or writing code, all contributions are appreciated.

To get started:

- Fork the repo
- Create your feature branch (git checkout -b feature/fooBar)
- Commit your changes (git commit -am 'Add some fooBar')
- Push to the branch (git push origin feature/fooBar)
- Create a new Pull Request

## 🎉 Acknowledgments
TaskFly was built with ❤️ and a lot of Rust. Big shout-out to the Rust community for their amazing resources and support.

