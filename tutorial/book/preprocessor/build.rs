// SPDX-License-Identifier: Apache-2.0

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // Write an empty `index.txt` file if it doesn't exist so we can compile the
    // preprocessor without needing to generate `index.txt`.
    let index = Path::new("../../../index.txt");
    if !index.exists() {
        let mut index = File::create(index).unwrap();
        index.write_all(b"").unwrap();
    }
}
