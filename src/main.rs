use crate::reference::Reference;
use clap::{Parser, Subcommand};
use std::str::FromStr;

mod edit_distance;
mod reference;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

// TODO: define search syntax
// some selector ideas:
//  in:<book|testament|letters|gospels>
//     narrows a given search to only the book, testament, section, etc.
//  person:<person>
//     narrows a given search to only passages concerning the person.
//  topic:<topic>
//     narrows a given search to those pertaining to the topic.

// TODO: implement bible index
// - TODO: determine index structure

#[derive(Subcommand, Debug)]
enum Command {
    /// Search for some phrase, term, or topic
    Search {
        /// The term or terms to search for. Ex. atonement in:OT
        search_terms: Vec<String>,
    },
    /// View a specifid passage
    Reference {
        /// The passage to view. Ex. Romans 8:1-12
        passage: Vec<String>,
    },
    /// (Re)index corpus.
    Index,
    /// Download Specified Bible version.
    Download,
}

fn execute(command: Command) {
    match command {
        Command::Reference { passage } => {
            let input = passage.join(" ");
            let reference = Reference::from_str(&input);
            match reference {
                Ok(reference) => println!("{reference}"),
                Err(err) => println!("there was an error looking up your reference: {err:?}"),
            }
        }
        Command::Search { .. } => todo!(),
        Command::Index => todo!(),
        Command::Download => todo!(),
    }
}

fn main() {
    let cli = Cli::parse();
    execute(cli.command);
}
