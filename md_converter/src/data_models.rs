
/// Depending on the method of parsing, the number-of-spaces per indent is
/// often different. By default, like on GitHub, it is 2 spaces
pub const INDENT_SPACES: usize = 2;

/// A heading can be an h1, h2, etc, we need to know the number to render
/// it appropriately
type HeadingNumber = usize;

/// UnorderedLists and OrderedLists contains a set number of spaces to
/// identify its indentation
type Indentations = usize;

#[derive(PartialEq, Debug)]
pub enum MarkdownForm {
    Heading(HeadingNumber),
    UnorderedList(Indentations),
    OrderedList(Indentations),
    Code,
    PrewrittenHTML,
    PlainText
}

#[derive(PartialEq, Debug)]
pub struct MarkdownData {
    pub form: MarkdownForm,
    pub inner_data: String
}

