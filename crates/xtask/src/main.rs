use clap::{Parser, Subcommand};
use codegen::codegen_grammar_files;
use std::env;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::Command;

mod codegen;

// Stolen from https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    /// Commands to do with the mdBook.
    #[command(subcommand, name = "book")]
    Book(BookCommands),
    /// Commands to do with code generation.
    #[command(subcommand, name = "gen")]
    Codegen(CodegenCommands),
}

#[derive(Subcommand)]
enum BookCommands {
    /// Starts the mdBook development server
    #[command(name = "dev")]
    Dev,
}

#[derive(Subcommand)]
pub enum CodegenCommands {
    /// Generates all grammar and syntax source files.
    #[command(name = "grammar")]
    Grammar,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Book(book_command)) => match book_command {
            BookCommands::Dev => {
                let book_path = project_root().join("book");

                println!("{:?} is the thing", book_path);

                let status = Command::new("mdbook")
                    .arg("serve")
                    .arg("--open")
                    .current_dir(book_path)
                    .status()
                    .expect("Failed to execute `mdbook serve --open`");

                if !status.success() {
                    eprintln!("`mdbook serve --open` failed with status {:?}", status.code());
                }
            }
        },
        Some(Commands::Codegen(codegen_command)) => match codegen_command {
            CodegenCommands::Grammar => {
                codegen_grammar_files(codegen_command, project_root(), false)
            }
        },
        _ => {}
    }
}

/// Returns the path to the root directory of `denim` project.
fn project_root() -> PathBuf {
    let dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());

    PathBuf::from(dir).parent().unwrap().parent().unwrap().to_owned()
}

impl Display for CodegenCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodegenCommands::Grammar => write!(f, "gen grammar"),
        }
    }
}
