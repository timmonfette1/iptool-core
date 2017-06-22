/* iptool_core
 Rust program to do IP related tasks.

 Functions to convert IPv4 to binary.

 Author: Tim Monfette
 Version: 1.0.0
*/

use std::thread;
use std::sync::Arc;
use std::fmt::Write;

use utilities::helpers::str_to_int;
use super::validate::valid_ipv4;

// Function for turning IPv4 into binary
pub fn ipv4_binary(v4: String) -> String {
    let valid = valid_ipv4(&v4); 

    if valid == false {
        let err = "IPv4 address is not valid! Cannot translate this address";
        return err.to_owned();
    }
 
    let ret = process_ipv4(&v4);
    ret
}

// Function to process the IPv4 address
fn process_ipv4(v4: &str) -> String {
    let trimmed = v4.trim();

    if trimmed == "0.0.0.0" {
        let bin = "00000000.00000000.00000000.00000000";
        return bin.to_owned();
    }

    let split = trimmed.split(".").map(String::from);
    let vec = Arc::new(split.collect::<Vec<String>>());
  
    // Messing with concurrency - this is bad code
    let vec_ref = vec.clone();
    let a = thread::spawn(move || {dec_to_binary(&vec_ref[0])});
    let vec_ref = vec.clone();
    let b = thread::spawn(move || {dec_to_binary(&vec_ref[1])});
    let vec_ref = vec.clone();
    let c = thread::spawn(move || {dec_to_binary(&vec_ref[2])});
    let vec_ref = vec.clone();
    let d = thread::spawn(move || {dec_to_binary(&vec_ref[3])});

    let bin = format!("{}.{}.{}.{}", a.join().unwrap(),
        b.join().unwrap(), c.join().unwrap(), d.join().unwrap());

    bin
}

// Function to convert decimal to binary
fn dec_to_binary(octet: &str) -> String {
    let mut o = str_to_int(octet);
    let mut result = String::new();
    let mut remain: i32;
    let mut val = 128;
    let mut count = 1;

    loop {
        remain = o - val;
       
        if remain > 0 {
            write!(&mut result, "1").unwrap();
            o = remain;
        } else if remain < 0 {
            write!(&mut result, "0").unwrap();
        } else if remain == 0 {
            let zeros = print_zeros(8 - count);
            write!(&mut result, "1{}", zeros).unwrap();
            break;
        }

        val = val / 2;
        count = count + 1;
    }

    result
}

// Print zeros once a remainder of zero
fn print_zeros(count: i32) -> String {
    let mut z = String::new();

    for _ in 0..count {
        write!(&mut z, "0").unwrap();
    }

    z
}
