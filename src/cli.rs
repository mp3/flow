use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// Task title
        title: String,
        
        /// Priority (low, medium, high, critical)
        #[arg(short, long, default_value = "medium")]
        priority: String,
        
        /// Due date (natural language, e.g., "tomorrow", "in 2 days")
        #[arg(short, long)]
        due: Option<String>,
    },
    
    /// List tasks
    Ls {
        /// Show all tasks (ignore context)
        #[arg(short, long)]
        all: bool,
        
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
    },
    
    /// Mark a task as done
    Done {
        /// Task ID
        id: i64,
    },
    
    /// Remove a task
    Rm {
        /// Task ID
        id: i64,
    },
    
    /// Open the Terminal UI
    Ui,
}
