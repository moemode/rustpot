use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader, Error, Read};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub struct LineMatcher<R: Read> {
    reader: BufReader<R>,
    pattern: String,
}

impl<R: Read> LineMatcher<R> {
    pub fn new(reader: R, pattern: &str) -> Self {
        LineMatcher {
            reader: BufReader::new(reader),
            pattern: pattern.to_string(),
        }
    }
}

impl<R: Read> Iterator for LineMatcher<R> {
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None, // End of file
            Ok(_) => {
                if line.contains(&self.pattern) {
                    Some(Ok(line))
                } else {
                    self.next() // Recursively call next to skip non-matching lines
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

pub fn search_in_reader<R: Read>(reader: R, pattern: &str) {
    let matcher = LineMatcher::new(reader, pattern);
    for line in matcher {
        match line {
            Ok(line) => print!("{}", line),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

pub fn search_in_file(filename: &str, pattern: &str) -> Result<(), std::io::Error> {
    let file = File::open(filename)?;
    search_in_reader(file, pattern);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_line_matcher_with_hello_pattern() {
        let data = "hello\nworld\nhello world\n";
        let cursor = Cursor::new(data);
        let pattern = "hello";
        let matcher = LineMatcher::new(cursor, pattern);
        let expected_lines = vec!["hello\n".to_string(), "hello world\n".to_string()];
        let result_lines: Vec<String> = matcher.map(|line| line.unwrap()).collect();
        assert_eq!(result_lines, expected_lines);
    }

    #[test]
    fn test_line_matcher_empty_input() {
        let data = "";
        let cursor = Cursor::new(data);
        let pattern = "hello";
        let matcher = LineMatcher::new(cursor, pattern);
        let expected_lines: Vec<String> = vec![];
        let result_lines: Vec<String> = matcher.map(|line| line.unwrap()).collect();
        assert_eq!(result_lines, expected_lines);
    }

    #[test]
    fn test_line_matcher_no_matches() {
        let data = "world\nfoo\nbar\n";
        let cursor = Cursor::new(data);
        let pattern = "hello";
        let matcher = LineMatcher::new(cursor, pattern);
        let expected_lines: Vec<String> = vec![];
        let result_lines: Vec<String> = matcher.map(|line| line.unwrap()).collect();
        assert_eq!(result_lines, expected_lines);
    }

    #[test]
    fn test_line_matcher_single_match() {
        let data = "world\nhello\nfoo\n";
        let cursor = Cursor::new(data);
        let pattern = "hello";
        let matcher = LineMatcher::new(cursor, pattern);
        let expected_lines = vec!["hello\n".to_string()];
        let result_lines: Vec<String> = matcher.map(|line| line.unwrap()).collect();
        assert_eq!(result_lines, expected_lines);
    }

    #[test]
    fn test_line_matcher_multiple_matches() {
        let data = "hello\nworld\nhello world\nhello\n";
        let cursor = Cursor::new(data);
        let pattern = "hello";
        let matcher = LineMatcher::new(cursor, pattern);
        let expected_lines = vec![
            "hello\n".to_string(),
            "hello world\n".to_string(),
            "hello\n".to_string(),
        ];
        let result_lines: Vec<String> = matcher.map(|line| line.unwrap()).collect();
        assert_eq!(result_lines, expected_lines);
    }

    #[test]
    fn test_line_matcher_pattern_at_different_positions() {
        let data = "hello world\nworld hello\nfoo hello bar\n";
        let cursor = Cursor::new(data);
        let pattern = "hello";
        let matcher = LineMatcher::new(cursor, pattern);
        let expected_lines = vec![
            "hello world\n".to_string(),
            "world hello\n".to_string(),
            "foo hello bar\n".to_string(),
        ];
        let result_lines: Vec<String> = matcher.map(|line| line.unwrap()).collect();
        assert_eq!(result_lines, expected_lines);
    }

    #[test]
    fn test_line_matcher_special_characters() {
        let data = "hello\nworld\nhello\nfoo\n";
        let cursor = Cursor::new(data);
        let pattern = "hello";
        let matcher = LineMatcher::new(cursor, pattern);
        let expected_lines = vec!["hello\n".to_string(), "hello\n".to_string()];
        let result_lines: Vec<String> = matcher.map(|line| line.unwrap()).collect();
        assert_eq!(result_lines, expected_lines);
    }

    #[test]
    fn test_line_matcher_case_sensitivity() {
        let data = "Hello\nhello\nHELLO\n";
        let cursor = Cursor::new(data);
        let pattern = "hello";
        let matcher = LineMatcher::new(cursor, pattern);
        let expected_lines = vec!["hello\n".to_string()];
        let result_lines: Vec<String> = matcher.map(|line| line.unwrap()).collect();
        assert_eq!(result_lines, expected_lines);
    }
}
