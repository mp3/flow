# Note-Taking Feature Documentation

## 1. Business Requirement Document (BRD)

### 1.1 Executive Summary
The "Flow" CLI tool currently excels at task management, helping developers stay in the zone. However, developers also need to capture non-actionable information—ideas, code snippets, meeting notes, and reference material—without leaving their terminal environment. This feature adds a lightweight note-taking capability to "Flow", further reducing context switching.

### 1.2 Problem Statement
- **Context Switching:** Developers currently have to switch to a separate app (Notion, Obsidian, Apple Notes) or create ad-hoc text files to write down notes. This breaks the "flow state".
- **Disconnected Information:** Notes are often scattered and not linked to the project context where they are relevant.
- **Friction:** Existing CLI note tools might not integrate well with the existing task workflow.

### 1.3 Goals
- **Unified Workflow:** Allow users to manage both tasks and notes within the same CLI tool.
- **Context Awareness:** Notes should be automatically linked to the current git repository (project), just like tasks.
- **Speed:** Capturing a note should be as fast as adding a task.

### 1.4 Target Audience
- Existing "Flow" users (Developers, DevOps, SysAdmins).
- Users looking for a terminal-centric "second brain" or scratchpad.

### 1.5 Key Requirements
- **Quick Capture:** `flow note add "Idea"` should be instant.
- **Project Scoping:** `flow note ls` should show notes relevant to the current directory by default.
- **Simplicity:** Focus on plain text/markdown. No complex formatting needed initially.

---

## 2. Product Requirement Document (PRD)

### 2.1 Features

#### 2.1.1 Core Note Operations (CRUD)
- **Add Note:** Create a new note with a title and optional content.
    - Command: `flow note add <title> [content]`
    - Options: `-t/--tags` to add tags.
- **List Notes:** View a list of notes.
    - Command: `flow note ls`
    - Default behavior: Show notes for the current project.
    - Options: `--all` to show global notes.
- **Show Note:** View the full content of a note.
    - Command: `flow note show <id>`
- **Edit Note:** Modify an existing note.
    - Command: `flow note edit <id>`
    - Options: `--title <new_title>`, `--content <new_content>`, `--tags <new_tags>`
- **Delete Note:** Remove a note.
    - Command: `flow note rm <id>`

#### 2.1.2 Data Model
- **Note Entity:**
    - `id`: Unique identifier (integer).
    - `title`: Short summary or headline.
    - `content`: Detailed body (text/markdown).
    - `project_path`: Path to the git repo (context).
    - `created_at`: Timestamp.
    - `tags`: List of strings (e.g., `@idea`, `@snippet`).

#### 2.1.3 Technical Implementation
- **Storage:** New SQLite table `notes`.
- **CLI Structure:** New subcommand `Note` under the main `flow` command.
    - `flow note <subcommand>`

### 2.2 User Stories
1.  **Idea Capture:** "As a dev, I want to quickly save an idea for a refactor without creating a formal task, so I don't forget it."
2.  **Snippet Saving:** "As a dev, I want to save a complex command I just ran so I can reference it later in this project."
3.  **Project Context:** "As a dev, when I return to a project, I want to see the notes I left for myself alongside my tasks."

### 2.3 Non-Functional Requirements
- **Performance:** Listing notes should be instant (< 50ms).
- **Consistency:** The command structure should mirror the existing `flow task` (or root commands) style.
