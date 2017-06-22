/* iptool_core
 Rust program to do IP related tasks.

 Set of helper functions.
 Common functions needed by multiple files

 Author: Tim Monfette
 Version: 1.0.0
*/

// Convert String to Int
pub fn str_to_int(input: &str) -> i32 {
    let input_int: i32 = input.trim().parse().expect("Cannot parse String to Int");
    input_int
}

// Pad a <4 chunk with zeros
pub fn pad_with_zeros(chunk: &str) -> String {
    let mut zero = String::new();
    let mut done = false;
    let length = 4 - chunk.len();

    while !done {
        zero.push('0');

        if zero.len() == length {
            done = true;
        }
    }

    let res = format!("{}{}", zero, chunk);

    res
}
