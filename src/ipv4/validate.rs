/* iptool_core
 Rust program to do IP related tasks.

 Functions to validate an IPv4 address.

 Author: Tim Monfette
 Version: 1.0.0
*/

use regex::Regex;

// Function for validating an IPv4 address
pub fn validate_ipv4(address: String) -> String {
    let valid = valid_ipv4(&address);
    let mut ret = "The IPv4 address you entered is invalid";

    if valid == true {
        ret = "The IPv4 address you entered is valid";
    }

    ret.to_owned()
}

// Test for valid IPv4
pub fn valid_ipv4(addr: &str) -> bool {
    let re = Regex::new(r"^(25[0-5]|2[0-4]\d|[0-1]?\d?\d)(\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)){3}$").unwrap();
    let trimmed = addr.trim();

    let check = re.is_match(&trimmed);
    let valid = check;

    valid
}
