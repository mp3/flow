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

    /// Manage notes
    #[command(subcommand)]
    Note(NoteCommands),
}

#[derive(Subcommand)]
pub enum NoteCommands {
    /// Add a new note
    Add {
        /// Note title
        title: String,

        /// Note content
        #[arg(short, long)]
        content: Option<String>,

        /// Tags (comma separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
    },

    /// List notes
    Ls {
        /// Show all notes (ignore context)
        #[arg(short, long)]
        all: bool,
    },

    /// Show a note
    Show {
        /// Note ID
        id: i64,
    },

    /// Remove a note
    Rm {
        /// Note ID
        id: i64,
    },

    /// Edit a note
    Edit {
        /// Note ID
        id: i64,

        /// New title
        #[arg(short, long)]
        title: Option<String>,

        /// New content
        #[arg(short, long)]
        content: Option<String>,

        /// New tags (comma separated)
        #[arg(short = 'g', long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
}
