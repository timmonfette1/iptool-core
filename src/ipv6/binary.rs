/* iptool_core
 Rust program to do IP related tasks.

 Functions to convert IPv6 to binary.

 Author: Tim Monfette
 Version: 1.0.0
*/

use std::fmt::Write;

use utilities::helpers::{str_to_int, pad_with_zeros};
use super::validate::valid_ipv6;

// Look-up for Hex to Binary
static BIN: [&'static str; 16] = ["0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000", "1001", "1010", "1011", "1100", "1101", "1110", "1111"];

// Function for turning IPv6 into Binary
pub fn ipv6_binary(v6: String) -> String {
    let valid = valid_ipv6(&v6);
    if valid == false {
        let err = "IPv6 address is not valid! Cannot translate this address";
        return err.to_owned();
    }

    let ret = process_ipv6(&v6);
    ret
}

// Function to process the IPv6 address
fn process_ipv6(v6: &str) -> String {

    // Account for empty address
    if v6.trim() == "::" {
        let empty = print_zeros(8);
        let new_emp = trim_result(&empty);
        return new_emp.to_owned();
    }

    let split = v6.split(":");
    let vec = split.collect::<Vec<&str>>();
    let mut length = vec.len();

    // Logic to account for address ending in ::
    if vec[length - 1].trim().len() == 0 {
        length = length - 1;
    }

    // Logic to account for address starting in ::
    let mut start: usize = 0;
    let zero_chunks: usize;
    let mut result = String::new();

    if vec[0].len() == 0 && vec[1].len() == 0 {
        start = 1;
        zero_chunks = 8 - length + 2;
    } else {
        zero_chunks = 8 - length + 1;
    }

    // Process each 16 bits
    for x in start..length {
        let byte = vec[x].trim();

        if byte.len() == 0 {
            write!(&mut result, "{}", print_zeros(zero_chunks as i32)).unwrap();
        } else if byte.len() < 4 && byte.len() > 0 {
            let zeros = pad_with_zeros(&byte);
            write!(&mut result, "{}:\n", hex_to_binary(&zeros)).unwrap();
        } else {
            write!(&mut result, "{}:\n", hex_to_binary(&byte)).unwrap(); 
        }
    }

    let new_res = trim_result(&result);
    
    new_res.to_owned()
}

// Convert 16 bit chunk to binary
fn hex_to_binary(chunk: &str) -> String {
    let char_vec:Vec<char> = chunk.chars().collect();
    let mut num: i32;
    let mut result = String::new();

    for x in 0..4 {
        let byte = char_vec[x];

        match byte {
            'a' | 'A' => num = 10,
            'b' | 'B' => num = 11,
            'c' | 'C' => num = 12,
            'd' | 'D' => num = 13,
            'e' | 'E' => num = 14,
            'f' | 'F' => num = 15,
            _         => num = str_to_int(&byte.to_string()),
        }

        write!(&mut result, "{}", BIN[num as usize]).unwrap();
    }

    result
}

// Print a specified number of zero chunks
fn print_zeros(count: i32) -> String {
    let mut zeros = String::new();

    for _ in 0..count {
        write!(&mut zeros, "0000000000000000:\n").unwrap(); 
    }

    zeros
}

// Helper function to trim trailing :
fn trim_result(mut s: &str) -> &str {
    while s.ends_with(":\n") {
        let l = s.len();
        let new_l = l.saturating_sub(":\n".len());
        s = &s[..new_l];
    }

    s
}
