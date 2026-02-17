mod agent;
mod config;
mod diff;
mod fallback;
mod prompt;

use clap::Parser;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "ai-mergetool", about = "AI-powered git merge tool")]
struct Cli {
    /// Path to the BASE file (common ancestor)
    base: PathBuf,
    /// Path to the LOCAL file (current branch)
    local: PathBuf,
    /// Path to the REMOTE file (incoming changes)
    remote: PathBuf,
    /// Path to the MERGED output file
    merged: PathBuf,

    /// Path to configuration file (default: ~/.ai-mergetool-config.json)
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let config = config::load_or_create(cli.config.as_deref());

    eprintln!("ai-mergetool: resolving {}", cli.merged.display());
    eprintln!(
        "  BASE={} LOCAL={} REMOTE={}",
        cli.base.display(),
        cli.local.display(),
        cli.remote.display()
    );

    let prompt = match prompt::build(&config, &cli.base, &cli.local, &cli.remote, &cli.merged) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error building prompt: {}", e);
            process::exit(1);
        }
    };

    match agent::invoke(&config, &prompt) {
        Ok(result) => {
            println!("{}", result.stdout);

            let local_content =
                std::fs::read_to_string(&cli.local).unwrap_or_default();
            let merged_content =
                std::fs::read_to_string(&cli.merged).unwrap_or_default();
            diff::print_colored_diff(&local_content, &merged_content);

            eprintln!("Merge result written to {}", cli.merged.display());
        }
        Err(e) => {
            eprintln!("Agent failed: {}", e);
            eprintln!("Falling back to traditional merge tool...");

            if let Err(e) =
                fallback::run(&config, &cli.base, &cli.local, &cli.remote, &cli.merged)
            {
                eprintln!("Fallback failed: {}", e);
                process::exit(1);
            }
        }
    }
}
