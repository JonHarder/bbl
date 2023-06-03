use std::str::FromStr;

use clap::{Parser, Subcommand};

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
}

#[derive(Debug)]
struct Chapter(u32);

#[derive(Debug)]
struct Verse(u32);

impl FromStr for Verse {
    type Err = PassageParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: u32 = s
            .parse()
            .map_err(|err| PassageParseError::BadFormat(format!("{:?}", err)))?;
        Ok(Verse(v))
    }
}

impl FromStr for Chapter {
    type Err = PassageParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: u32 = s
            .parse()
            .map_err(|err| PassageParseError::BadFormat(format!("{:?}", err)))?;
        Ok(Chapter(v))
    }
}

/// Defines a passage which contains a start reference, and optionally an
/// end reference
#[derive(Debug)]
struct Passage {
    start_book: String,
    start_chapter: Option<Chapter>,
    start_verse: Option<Verse>,
}

#[derive(Debug)]
enum PassageParseError {
    MissingBook,
    MissingChapter,
    BadFormat(String),
}

impl Passage {
    fn parse(input: Vec<String>) -> Result<Self, PassageParseError> {
        if input.is_empty() {
            return Err(PassageParseError::MissingBook);
        }
        let book = input[0].to_string();
        match input.get(1) {
            Some(rest) => {
                if let Some((chapter, rest)) = rest.split_once(":") {
                    let chapter = chapter.parse()?;
                    let verse = rest.parse()?;
                    Ok(Passage {
                        start_book: book,
                        start_chapter: Some(chapter),
                        start_verse: Some(verse),
                    })
                } else {
                    let chapter = rest.parse()?;
                    Ok(Passage {
                        start_book: book,
                        start_chapter: Some(chapter),
                        start_verse: None,
                    })
                }
            }
            None => Ok(Passage {
                start_book: book,
                start_chapter: None,
                start_verse: None,
            }),
        }
    }
}

fn execute(command: Command) {
    match command {
        Command::Reference { passage } => {
            let p = Passage::parse(passage);
            println!("{p:?}");
        }
        Command::Search { .. } => todo!(),
        Command::Index => todo!(),
    }
}

fn main() {
    let cli = Cli::parse();
    execute(cli.command);
}
