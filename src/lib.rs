pub mod binary_search;
pub mod binary_tree;
pub mod grid;
pub mod segment_tree;
pub mod union_find;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const BINARY_TREE_SRC: &'static str = include_str!("binary_tree.rs");
pub const GRID_SRC: &'static str = include_str!("grid.rs");
pub const UNION_FIND_SRC: &'static str = include_str!("union_find.rs");
pub const BINARY_SEARCH_SRC: &'static str = include_str!("binary_search.rs");
pub const SEGMENT_TREE_SRC: &'static str = include_str!("segment_tree.rs");

pub fn get_module_source(str: String) -> &'static str {
    if str.starts_with("binary_tree") {
        BINARY_TREE_SRC
    } else if str.starts_with("grid") {
        GRID_SRC
    } else if str.starts_with("union_find") {
        UNION_FIND_SRC
    } else if str.starts_with("binary_search") {
        BINARY_SEARCH_SRC
    } else if str.starts_with("segment_tree") {
        SEGMENT_TREE_SRC
    } else {
        panic!("unknown module for cauly_rust_leetcode_util");
    }
}
