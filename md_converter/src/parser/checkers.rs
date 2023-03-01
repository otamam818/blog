use std::collections::HashSet;

pub struct SyntaxChecker;
impl SyntaxChecker {
    #[allow(dead_code)]
    pub fn md_chars() -> HashSet<char> {
        HashSet::from([
            // Markdown's basic syntax, as outlined in John Gruber’s original
            // design document. All Markdown applications support these elements
            '#', '-', '`', '*', '>', '[', '!',
            // Support for numbers
            '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'
        ])
    }

    pub fn numbers() -> HashSet<char> {
        HashSet::from(['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'])
    }
}

pub fn is_title(line: &str) -> bool {
    line.starts_with("#")
}

