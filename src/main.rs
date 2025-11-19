mod cli;
mod context;
mod db;
mod models;
mod tui;

use clap::Parser;
use colored::*;
use anyhow::Result;
use crate::cli::{Args, Commands};
use crate::db::TaskRepository;
use crate::models::{Task, Priority, Status};
use crate::context::ContextManager;
use chrono::Local;
use chrono_english::{parse_date_string, Dialect};

fn main() -> Result<()> {
    let args = Args::parse();
    let repo = TaskRepository::init()?;

    match args.command {
        Some(Commands::Add { title, priority, due }) => {
            let context_path = ContextManager::get_context()?;
            let project_path = context_path.to_string_lossy().to_string();
            
            let due_date = if let Some(d) = due {
                Some(parse_date_string(&d, Local::now(), Dialect::Us)?)
            } else {
                None
            };

            let task = Task {
                id: None,
                title,
                description: None,
                status: Status::Todo,
                priority: Priority::from(priority),
                due_date,
                project_path: Some(project_path.clone()),
                created_at: Local::now(),
                tags: Vec::new(),
            };

            let id = repo.add_task(&task)?;
            println!("{} Task added with ID: {}", "✔".green(), id);
            println!("   Context: {}", project_path.dimmed());
        }
        Some(Commands::Ls { all, status }) => {
            let context_path = if all {
                None
            } else {
                Some(ContextManager::get_context()?)
            };
            
            let filter_path = context_path.as_ref().map(|p| p.to_string_lossy());
            let tasks = repo.get_tasks(filter_path.as_deref())?;

            if tasks.is_empty() {
                println!("No tasks found.");
                return Ok(());
            }

            println!("{:<4} {:<30} {:<10} {:<10} {:<20}", "ID", "Title", "Status", "Priority", "Due");
            println!("{}", "-".repeat(80));

            for task in tasks {
                if let Some(s) = &status {
                    if task.status.to_string().to_lowercase() != s.to_lowercase() {
                        continue;
                    }
                }

                let title = if task.status == Status::Done {
                    task.title.strikethrough()
                } else {
                    task.title.normal()
                };

                let priority = match task.priority {
                    Priority::High | Priority::Critical => task.priority.to_string().red(),
                    Priority::Medium => task.priority.to_string().yellow(),
                    Priority::Low => task.priority.to_string().green(),
                };

                let due = match task.due_date {
                    Some(d) => d.format("%Y-%m-%d %H:%M").to_string(),
                    None => "-".to_string(),
                };

                println!("{:<4} {:<30} {:<10} {:<10} {:<20}", 
                    task.id.unwrap_or(0), 
                    title, 
                    task.status, 
                    priority, 
                    due
                );
            }
        }
        Some(Commands::Done { id }) => {
            repo.complete_task(id)?;
            println!("{} Task {} marked as done.", "✔".green(), id);
        }
        Some(Commands::Rm { id }) => {
            repo.delete_task(id)?;
            println!("{} Task {} removed.", "✔".green(), id);
        }
        Some(Commands::Ui) => {
            tui::run(&repo)?;
        }
        None => {
            // Default to TUI if no command
            tui::run(&repo)?;
        }
    }

    Ok(())
}
