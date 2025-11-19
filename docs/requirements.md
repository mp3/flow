# Requirements Document: "Flow" - The CLI Task Manager

## 1. Business Requirement Document (BRD)

### 1.1 Executive Summary
"Flow" is a high-performance, Rust-based CLI task management tool designed specifically for developers who live in the terminal. The goal is to minimize context switching and maximize "flow state" by integrating task management seamlessly into the developer's workflow.

### 1.2 Problem Statement
Existing task managers (web-based or GUI) require leaving the terminal, breaking concentration. Existing CLI tools often lack modern UX, are slow, or have steep learning curves. Developers need a tool that feels like a natural extension of their shell.

### 1.3 Target Audience
- Software Engineers
- DevOps Engineers
- System Administrators
- Anyone who spends significant time in a terminal environment.

### 1.4 Key Differentiators
1.  **Blazing Fast:** Written in Rust for instant startup and execution. No JVM or Python interpreter lag.
2.  **Context-Aware:** Automatically detects the current project (git repo) and filters tasks accordingly.
3.  **Frictionless Input:** Natural language parsing for dates and priorities (e.g., `flow add "Fix bug" tomorrow !high`).
4.  **Git Integration:** Can create branches from tasks and suggest commit messages based on completed tasks.
5.  **Offline First:** All data stored locally (SQLite or JSON), ensuring privacy and availability.

---

## 2. Product Requirement Document (PRD)

### 2.1 Core Features

#### 2.1.1 Task Management (CRUD)
- **Add:** Create tasks with title, description, due date, priority, and tags.
- **List:** View tasks with flexible filtering (by status, priority, tag, project).
- **Done:** Mark tasks as complete.
- **Edit:** Modify existing tasks.
- **Delete:** Remove tasks.

#### 2.1.2 Context & Organization
- **Project Scoping:** Tasks are linked to the current directory/git repository by default. Global tasks are also supported.
- **Tags:** Flexible tagging system (e.g., `@bug`, `@feature`, `@urgent`).
- **Priorities:** Simple priority levels (Low, Medium, High, Critical).

#### 2.1.3 User Interface (CLI & TUI)
- **Command Mode:** Standard CLI arguments for scriptability and quick entry.
    - `flow add "Deploy to prod"`
    - `flow ls`
- **Interactive Mode (TUI):** A rich terminal UI for browsing and managing tasks visually using keyboard shortcuts (vim-style navigation).
    - Invoked via `flow ui` or just `flow` (configurable).

#### 2.1.4 Smart Features
- **Natural Language Processing:** Parse relative dates ("next monday", "in 2 hours").
- **Git Workflow:**
    - `flow start <task_id>`: Switches to a new git branch named after the task.

### 2.2 Technical Requirements

- **Language:** Rust (stable).
- **Storage:** SQLite (for reliability and query power) or plain JSON (for portability/git-friendliness). *Recommendation: SQLite for performance/scale, with import/export to JSON.*
- **Libraries:**
    - `clap`: Command line argument parsing.
    - `ratatui`: Terminal UI.
    - `sqlx` or `rusqlite`: Database interaction.
    - `chrono`: Date and time handling.

### 2.3 User Stories

1.  **Quick Capture:** As a dev, I want to add a task immediately when I think of it without leaving my code editor, so I don't lose focus.
2.  **Daily Standup:** As a dev, I want to see what I completed yesterday and what I plan to do today, so I can give a quick update.
3.  **Context Switch:** As a dev switching to a different repo, I want to see only tasks relevant to that repo.

### 2.4 Success Metrics
- **Performance:** < 20ms startup time.
- **Usability:** A new user can add and list tasks within 1 minute of installation.
