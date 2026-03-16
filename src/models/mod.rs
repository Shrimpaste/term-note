pub mod note;
pub mod search;

pub use note::Note;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewState {
    List,
    View,
    Edit,
    Search,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditMode {
    Normal,
    Insert,
}