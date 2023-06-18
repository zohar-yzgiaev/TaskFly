# 🚀 TaskFly - The Fun Cron Alternative

TaskFly allows you to schedule tasks according to their urgency level, represented by fun emojis! So why learn the cryptic cron syntax when you can just speak the language of emojis?

## 🤔 What is TaskFly?

TaskFly is a command-line tool designed to make task scheduling easy and fun. Instead of wrangling with cron syntax, you simply assign your tasks an urgency level using one of four emojis. TaskFly will then ensure your tasks are executed at the right time based on their urgency.

Here are the available urgency levels:

- 🚀 Blast Off: Super urgent tasks that need to be executed within the next 30 minutes.
- ✈️ Jet Stream: Urgent tasks that need to be executed within the next 2 hours.
- 🚲 Daily Cruise: Tasks that need to be executed by the end of the day.
- 🐢 Turtle Pace: Tasks that can wait to be executed sometime this week.

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

