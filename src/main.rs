use std::{
    fs::{self, OpenOptions},
    io::prelude::*,
};

use anyhow::{anyhow, Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = fs::read_to_string(args.file)?;

    let blocks = fold_blocks(&file).expect("Invalid Syntax");

    write_block(blocks)
        .context("asdf")?
        .into_iter()
        .for_each(|file| {
            file.map_or_else(|err| println!("{err}"), |file| println!("{file}"));
        });

    Ok(())
}

enum Block {
    Ready(Vec<(String, String)>),
    InProgress(((String, String), Vec<(String, String)>)),
    Complete(Vec<(String, String)>),
}

fn get_path(line: &str) -> Option<String> {
    let mut words = line.split(' ');
    while let Some(word) = words.next() {
        if word == ":tangle" {
            return Some(words.next().unwrap_or_default().to_string());
        }
    }
    None
}

fn remove_element<T: PartialEq, U>(vec: &mut Vec<(T, U)>, value: &T) -> Option<U> {
    vec.iter()
        .position(|(x, _)| x == value)
        .map(|pos| vec.remove(pos).1)
}

// TODO: As this generalizes this will become SrcBlock and may store additional metadata.
// The below fold_blocks functions will likely be rolled into ::new(file: &str)

impl Block {
    fn new() -> Block {
        Block::Ready(Vec::new())
    }
    fn insert(self, line: &str) -> Block {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            match self {
                // ``` in progress will end the block
                Block::InProgress(((path, mut content), mut entries)) => {
                    content.push('\n');
                    entries.push((path, content));
                    Block::Ready(entries)
                }
                // ``` while ready generally starts a block. Check for tangle otherwise ignore
                Block::Ready(mut entries) => {
                    let path = get_path(trimmed);
                    if path.is_some() {
                        let path = path.expect("Already Confirmed. Failure Unreachable.");
                        if let Some(content) = remove_element(&mut entries, &path) {
                            Block::InProgress(((path, content), entries))
                        } else {
                            Block::InProgress(((path, String::new()), entries))
                        }
                    } else {
                        Block::Ready(entries)
                    }
                }
                // Fallthrough case no entries just return self.
                _ => self,
            }
        } else {
            match self {
                // If currently building a block just push line content and continue
                Block::InProgress(((path, mut content), entries)) => {
                    content.push_str(line);
                    Block::InProgress(((path, content), entries))
                }
                // If not builing a block just proceed.
                _ => self,
            }
        }
    }
    fn complete(self) -> Result<Block> {
        match self {
            Block::Ready(entries) => Ok(Block::Complete(entries)),
            _ => Err(anyhow!(
                "Cannot complete a block of type Complete or InProgress."
            )),
        }
    }
}

fn fold_blocks(file: &str) -> Result<Block> {
    file.lines()
        .fold(Block::new(), |acc, line| acc.insert(line))
        .complete()
}

fn write_block(blocks: Block) -> Result<Vec<Result<String>>> {
    match blocks {
        Block::Complete(entries) => Ok(entries
            .into_iter()
            .map(|(file, content)| {
                let true_path = determine_path(file).context("File invalid");
                (true_path, content)
            })
            .map(|(path, content)| {
                if let Ok(path) = path {
                    if write_file(path.clone(), content).is_err() {
                        Err(anyhow!("Failed to write {path}"))
                    } else {
                        Ok(format!("Saved {path}").to_string())
                    }
                } else {
                    Err(anyhow!("Invalid path"))
                }
            })
            .collect()),
        _ => Err(anyhow!("Parsing InComplete")),
    }
}

// TODO: Should be simple absolute and relative paths should be accepted. (Pass args?
// std::env::args should give me the full path of the file. Want to used named arguments though for
// adding additional tooling to the CLI. (DB sync (with sentiment/context vectors), schedule
// parsing on save, updating original file for comment syntax on task timelines etc...))
fn determine_path(_path: String) -> Result<String> {
    todo!("Not done.")
}

fn write_file(path: String, content: String) -> Result<()> {
    let mut file = OpenOptions::new().write(true).open(path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
