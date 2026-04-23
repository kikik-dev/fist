use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "fist", about = "Fist Framework CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { name: String },
    /// Run the server
    Serve {
        #[arg(short, long)]
        watch: bool,
    },
    /// Build the project for production
    Build {
        #[arg(short, long)]
        release: bool,
    },
    Make {
        #[command(subcommand)]
        generator: Generator,
    },
}

#[derive(Subcommand)]
enum Generator {
    Controller { name: String },
    Service { name: String },
    Model { name: String, #[arg(short, long)] migration: bool },
    Dto { name: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => println!("Creating new project: {}", name),
        Commands::Serve { watch } => {
            if watch {
                println!("Starting Fist server with hot-reload...");
                Command::new("cargo")
                    .args(["watch", "-x", "run"])
                    .status()
                    .expect("Failed to start cargo-watch");
            } else {
                println!("Starting Fist server...");
                Command::new("cargo")
                    .args(["run", "--release"])
                    .status()
                    .expect("Failed to run server");
            }
        },
        Commands::Build { release } => {
            let mut args = vec!["build"];
            if release {
                args.push("--release");
            }
            println!("Building project...");
            Command::new("cargo")
                .args(&args)
                .status()
                .expect("Failed to build project");
        },
        Commands::Make { generator } => match generator {
            Generator::Controller { name } => println!("Generating controller: {}", name),
            Generator::Service { name } => println!("Generating service: {}", name),
            Generator::Model { name, migration } => println!("Generating model: {}, migration: {}", name, migration),
            Generator::Dto { name } => println!("Generating DTO: {}", name),
        },
    }
}
