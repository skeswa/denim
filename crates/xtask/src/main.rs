use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;
use std::process::Command;

// Stolen from https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts the mdBook development server
    #[command(name = "book:dev")]
    BookDev,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::BookDev) => {
            let repo_path = find_repo_path("denim").expect("Failed to find repo path");

            let book_path = repo_path.join("book");

            let status = Command::new("mdbook")
                .arg("serve")
                .arg("--open")
                .current_dir(book_path)
                .status()
                .expect("Failed to execute `mdbook serve --open`");

            if !status.success() {
                eprintln!(
                    "`mdbook serve --open` failed with status {:?}",
                    status.code()
                );
            }
        }
        None => {}
    }
}

fn find_repo_path(repo_name: &str) -> Option<PathBuf> {
    let mut current_dir = env::current_dir().expect("Failed to get current directory");

    loop {
        if current_dir.ends_with(repo_name) {
            return Some(current_dir);
        }

        if !current_dir.pop() {
            break;
        }
    }

    None
}
