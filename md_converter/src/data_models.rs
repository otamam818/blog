
/// Depending on the method of parsing, the number-of-spaces per indent is
/// often different. By default, like on GitHub, it is 2 spaces
pub const INDENT_SPACES: usize = 2;

#[derive(PartialEq, Debug, Clone)]
pub enum MarkdownForm {
    Heading { heading_number: usize },
    List {
        indents: usize,
        is_ordered: bool,
        inner_bullet: Option<Box<MarkdownData>>
    },
    // TODO: Implement these
    // Code,
    // PrewrittenHTML,
    PlainText { has_line_break: bool }
}

impl MarkdownForm {
    pub fn new_plaintext() -> MarkdownForm {
        MarkdownForm::PlainText { has_line_break: true }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct MarkdownData {
    pub form: MarkdownForm,
    pub inner_data: String
}

