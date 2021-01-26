pub mod binary_tree;
pub mod grid;
pub mod union_find;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const BINARY_TREE_SRC: &'static str = include_str!("binary_tree.rs");
pub const GRID_SRC: &'static str = include_str!("grid.rs");
pub const UNION_FIND_SRC: &'static str = include_str!("union_find.rs");
