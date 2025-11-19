# Flow 🌊

**The CLI Task Manager for Developers.**

Flow is a high-performance, Rust-based task management tool designed to keep you in your "flow" state. It integrates seamlessly with your terminal workflow, offering context-aware task management that knows which git repository you're working in.

## 🚀 Features

- **⚡️ Blazing Fast**: Written in Rust for instant startup and execution.
- **🧠 Context Aware**: Automatically scopes tasks to your current Git repository.
- **📅 Natural Language**: Add tasks with "tomorrow", "next friday", or "in 2 hours".
- **🖥️ TUI Mode**: Beautiful interactive terminal UI for managing tasks.
- **💾 Local First**: All data stored locally in SQLite.

## 📦 Installation

```bash
cargo install --path .
```

## 📖 Usage

### Quick Add
Add a task with priority and due date:
```bash
flow add "Refactor database layer" --priority high --due "tomorrow"
```

### List Tasks
See tasks for your **current project**:
```bash
flow ls
```
See **all** tasks:
```bash
flow ls --all
```

### Interactive Mode
Launch the TUI:
```bash
flow ui
```
- `j`/`k`: Navigate
- `Space`: Toggle status
- `q`: Quit

## 🛠️ Tech Stack
- Rust
- SQLite (rusqlite)
- Ratatui (TUI)
- Clap (CLI)
