/// Struct for parsing and tokenizing scripture text.
/// NOT for parsing passage references.
#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer {
        Tokenizer { input }
    }

    pub fn tokenize(&self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_thing() {
        let tokenizer = Tokenizer::new("hello world");
    }
}
