use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use regex::Regex;

use crate::edit_distance::edit_distance;

#[derive(Debug, Clone)]
pub enum Book {
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    Samuel1,
    Samuel2,
    Kings1,
    Kings2,
    Matthew,
    Mark,
    Luke,
    John,
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Genesis => "Genesis",
            Book::Exodus => "Exodus",
            Book::Leviticus => "Leviticus",
            Book::Numbers => "Numbers",
            Book::Deuteronomy => "Deuteronomy",
            Book::Joshua => "Joshua",
            Book::Judges => "Judges",
            Book::Ruth => "Ruth",
            Book::Samuel1 => "1Samuel",
            Book::Samuel2 => "2Samuel",
            Book::Kings1 => "1Kings",
            Book::Kings2 => "2Kings",
            Book::Matthew => "Matthew",
            Book::Mark => "Mark",
            Book::Luke => "Luke",
            Book::John => "John",
        })
    }
}

const BOOKS: [Book; 16] = [
    Book::Genesis,
    Book::Exodus,
    Book::Leviticus,
    Book::Numbers,
    Book::Deuteronomy,
    Book::Joshua,
    Book::Judges,
    Book::Ruth,
    Book::Samuel1,
    Book::Samuel2,
    Book::Kings1,
    Book::Kings2,
    Book::Matthew,
    Book::Mark,
    Book::Luke,
    Book::John,
];

#[derive(Debug)]
pub struct Reference {
    pub book: Book,
    pub chapter: Option<u8>,
    pub verse: Option<u8>,
}

impl Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.chapter, self.verse) {
            (None, None) => write!(f, "{}", self.book),
            (Some(chapter), None) => write!(f, "{} {chapter}", self.book),
            (Some(chapter), Some(verse)) => write!(f, "{} {chapter}:{verse}", self.book),
            (None, Some(_)) => panic!("a verse without a chapter should not be possible"),
        }
    }
}

#[derive(Debug)]
pub enum ReferenceError {
    UnknownBook,
    InvalidReferenceFormat(&'static str),
}

#[derive(PartialEq, Eq)]
enum CaseSensitivity {
    CaseSensitive,
    CaseInsensitive,
}

fn find_closest<'a, T: ToString + Display>(
    input: &str,
    options: &'a [T],
    case_sensitive: CaseSensitivity,
) -> Option<&'a T> {
    let mut option_distances: Vec<_> = options
        .iter()
        .map(|o| {
            let option = if case_sensitive == CaseSensitivity::CaseSensitive {
                o.to_string()
            } else {
                o.to_string().to_uppercase()
            };
            let input = if case_sensitive == CaseSensitivity::CaseSensitive {
                input.to_uppercase()
            } else {
                input.to_string()
            };
            (edit_distance(&input, &option, None), o)
        })
        .collect();
    option_distances.sort_by(|(d1, _), (d2, _)| d1.total_cmp(d2));

    // for (dist, book) in &option_distances {
    //     println!("distance: {dist:.2}, book: {book}");
    // }

    Some(option_distances.first().unwrap().1)
}

impl FromStr for Book {
    type Err = ReferenceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // TODO: need to find a way to match on explicit book abbreviations as well.
        // e.g. jn for John.
        // mayke just check a static mapping first, and then if no hits,
        // use `find_closest`
        let value = value.to_uppercase();
        if value.is_empty() {
            return Err(ReferenceError::UnknownBook);
        }
        find_closest(&value, &BOOKS, CaseSensitivity::CaseInsensitive)
            .ok_or(ReferenceError::UnknownBook)
            .cloned()
    }
}

impl FromStr for Reference {
    type Err = ReferenceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let r_book_chapter_verse =
            Regex::new(r"^(?<book>\d?\s?[a-zA-Z]+) (?<chapter>\d+):(?<verse>\d+)$")
                .expect("this to be valid regex");
        let r_book_chapter = Regex::new(r"^(?<book>\d?\s?[a-zA-Z]+) (?<chapter>\d+)$")
            .expect("this to be valid regex");
        let r_book = Regex::new(r"^(?<book>\d?\s?[a-zA-Z]+)$").expect("this to be valid regex");

        if let Some(captures) = r_book_chapter_verse.captures(value) {
            let (_, [book, chapter, verse]) = captures.extract();
            let book = Book::from_str(book)?;
            let chapter = chapter.parse().unwrap();
            let verse = verse.parse().unwrap();
            Ok(Reference {
                book,
                chapter: Some(chapter),
                verse: Some(verse),
            })
        } else if let Some(captures) = r_book_chapter.captures(value) {
            let (_, [book, chapter]) = captures.extract();
            let book = Book::from_str(book)?;
            let chapter = chapter.parse().unwrap();
            Ok(Reference {
                book,
                chapter: Some(chapter),
                verse: None,
            })
        } else if let Some(captures) = r_book.captures(value) {
            let (_, [book]) = captures.extract();
            let book = Book::from_str(book)?;
            Ok(Reference {
                book,
                chapter: None,
                verse: None,
            })
        } else {
            Err(ReferenceError::InvalidReferenceFormat(
                "Failed to parse reference",
            ))
        }
    }
}
