use cauly_rust_leetcode_utils;
use std::env;
use std::fs;
use std::path::Path;

fn convert(str: String) -> &'static str {
    if str.starts_with("binary_tree") {
        cauly_rust_leetcode_utils::BINARY_TREE_SRC
    } else if str.starts_with("grid") {
        cauly_rust_leetcode_utils::GRID_SRC
    } else if str.starts_with("union_find") {
        cauly_rust_leetcode_utils::UNION_FIND_SRC
    } else {
        panic!("unknown module for cauly_rust_leetcode_util");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    if args.len() > 1 {
        filename = args[1].clone();
    } else if Path::new("lib.rs").is_file() {
        filename = "lib.rs".to_string();
    } else if Path::new("src/lib.rs").is_file() {
        filename = "src/lib.rs".to_string();
    } else {
        panic!("File name not specified, and no default lib.rs found.");
    }

    let content = fs::read_to_string(filename).unwrap();
    let mut result = String::new();

    for line in content.lines() {
        if line.starts_with("pub struct Solution;") || line.starts_with("struct Solution;") {
            // skip
        } else if line.starts_with("use cauly_rust_leetcode_utils::") {
            let remaining = line["use cauly_rust_leetcode_utils::".len()..].to_string();
            result.push_str("// These code are written and formatted by cauly-rust-leetcode-utils. See `https://github.com/CaulyKan/cauly-rust-leetcode-utils` for more infomation.\n");
            result.push_str(convert(remaining));
        } else {
            result.push_str(line);
            result.push_str("\n");
        }
    }

    println!("{}", result);
}
