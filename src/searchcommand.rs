use crate::searchtype::SearchType;

pub enum SearchCommand {
    Quit,
    FindBestMove(SearchType)
}