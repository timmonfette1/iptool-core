/* iptool_core
 Rust program to do IP related tasks.

 Functions to translate IPv4 to IPv6.

 Author: Tim Monfette
 Version: 1.0.0
*/

use std::thread;
use std::sync::Arc;

use utilities::helpers::str_to_int;
use super::validate::valid_ipv4;

// Look-up for Hex values of integers
static HEX: [&'static str; 16] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"];

// Function for IPv4 to IPv6
pub fn ipv4_to_ipv6(v4: String) -> String {
    let valid = valid_ipv4(&v4);
    if valid == false {
        let err = "IPv4 address is not valid! Cannot translate this address";
        return err.to_owned();
    }

    let split = v4.split(".").map(String::from);
    let vec = Arc::new(split.collect::<Vec<String>>());

    // Messing with concurrency - this is bad code
    let vec_ref = vec.clone();
    let a = thread::spawn(move || {dec_to_hex(&vec_ref[0])});
    let vec_ref = vec.clone();
    let b = thread::spawn(move || {dec_to_hex(&vec_ref[1])});
    let vec_ref = vec.clone();
    let c = thread::spawn(move || {dec_to_hex(&vec_ref[2])});
    let vec_ref = vec.clone();
    let d = thread::spawn(move || {dec_to_hex(&vec_ref[3])});

    let v6 = format!("2002:{}{}:{}{}::", a.join().unwrap(),
        b.join().unwrap(), c.join().unwrap(), d.join().unwrap());

    v6
}

// Convert Decimal to hex
fn dec_to_hex(octet: &str) -> String {
    let b = str_to_int(octet);
    let mut vec = vec!["0", "0"];

    if b <= 15 {
        let result = format!("0{}", HEX[b as usize]);
        return result
    } else if b % 16 == 0 {
        let result = format!("{}0", HEX[(b/16) as usize]);
        return result
    }

    let mut answer = b;
    let mut count = 1;

    while answer != 0 {
        let modulo = answer % 16;
        answer = answer / 16;
        vec[count] = HEX[(modulo) as usize];
        count = 0;
    }

    // Return the hex value
    let result = format!("{}{}",vec[0],vec[1]);
    result
}
