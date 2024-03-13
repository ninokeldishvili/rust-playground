use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).with_context(|| format!("Could not read file '{}'", &args.path.to_string_lossy()))?;

    println!("file content: {}", content);
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())

}