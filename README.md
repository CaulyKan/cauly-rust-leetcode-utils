# cauly-rust-leetcode-utils
This project provides some aids for writing leetcode in rust language.


## How to use
method 1: 
1. [Folk and] clone this repo.
2. in your leetcode cargo.toml, add:
   ```
   [dependencies]
    cauly-rust-leetcode-utils={path="path/to/cauly-rust-leetcode-utils"}
   ``` 
3. use data structures from cauly-rust-leetcode-utils ("use cauly_rust_leetcode_utils::..")
4. run `cargo run --manifest-path=path/to/cauly-rust-leetcode-utils/Cargo.toml | clip.exe` to format and copy code that goes into leetcode editor.

method 2 (for those who dont want to download cauly-rust-leetcode-utils):
1. run `cargo install cauly-rust-leetcode-utils`, this gives you a program called `rlfmt`
2. in your leetcode cargo.toml, add:
   ```
    [dependencies]
    cauly-rust-leetcode-utils="LATEST_VERSION"
   ```
3. use data structures from cauly-rust-leetcode-utils ("use cauly_rust_leetcode_utils::..")
4. run `rlfmt | clip.exe` to format and copy code that goes into leetcode editor.


## How rlfmt works
* rlfmt takes 1 parameter for input .rs file. If no parameter is provided, it looks for `lib.rs`, `src/lib.rs`, then fail.
* rlfmt will delete `pub struct Solution;` or `struct Solution;` which is not necessary for leetcode.
* rlfmt will look for `use cauly_rust_leetcode_utils::XXX` and replace with actual source code.
* finally rlfmt prints the result to output, so that you can redirect it to clipboard with `clip.exe` in windows, or `xsel -i` in linux.