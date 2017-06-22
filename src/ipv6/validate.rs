/* iptool_core
 Rust program to do IP related tasks.

 Functions to validate an IPv6 address.

 Author: Tim Monfette
 Version: 1.0.0
*/

use regex::Regex;
use std::cmp::Ordering;

use utilities::helpers::pad_with_zeros;

// Function for validating an IPv6 address
pub fn validate_ipv6(address: String) -> String {
    let valid = valid_ipv6(&address);
    let mut ret = "The IPv6 address you entered is invalid";

    if valid == true {
        ret = "The IPv6 address you entered is valid";
    }

    ret.to_owned()
}

// Test for valid IPv6
pub fn valid_ipv6(addr: &str) -> bool {
    let mut valid = true;
    let mut count = 0;
    let mut blanks = 0;
    let re = Regex::new(r"[0-9a-fA-F]{4}").unwrap();

    // Account for empty address
    if addr.trim() == "::" {
        return true;
    }

    // Account for address with no :
    if !addr.contains(":") {
        return false;
    }

    let split = addr.split(":");
    let vec = split.collect::<Vec<&str>>();

    // Account for an address ending in ::
    if vec[vec.len() - 1].trim().len() == 0 {
        blanks = -1;
    }

    // Account for an address leading or trailing :
    if vec[0].len() == 0 && vec[1].len() != 0 { 
        return false;
    } else if vec[vec.len() - 1].trim().len() == 0 && vec[vec.len() - 2].len() != 0 {
        return false;
    }

    // Process address
    for x in 0..vec.len() {
        let trimmed = vec[x].trim();

        // If :: shows up twice, bad address
        // Accounts for valid address like :: or ::1
        if trimmed.len() == 0 && count != 0 {
            blanks = blanks + 1;
            if blanks > 1 {
                valid = false;
                break;
            }
        }

        match trimmed.len().cmp(&4) {
            Ordering::Equal   => {
                let check = re.is_match(&trimmed);
                valid = check;
            }
            Ordering::Less    => {
                let zeros = pad_with_zeros(trimmed);
                let check = re.is_match(&zeros);
                valid = check;
            }
            Ordering::Greater => {
                valid = false;
            }
        }

        if valid == false {
            break;
        }

        count = count + 1;
        if count > 8 {
            valid = false;
            break;
        }
    }

    valid

}

// Validate an IPv4 address represented as IPv6
// Assumes valid formats are 2002::xxxx:xxxx or 2002:xxxx:xxxx::
pub fn validate_6to4(addr: &str) -> bool { 
    let re1 = Regex::new(r"^2002::[0-9a-fA-F]{4}:[0-9a-fA-F]{4}$").unwrap();
    let re2 = Regex::new(r"^2002:[0-9a-fA-F]{4}:[0-9a-fA-F]{4}::$").unwrap();

    let trimmed = addr.trim();

    let check1 = re1.is_match(&trimmed);
    let check2 = re2.is_match(&trimmed);

    if check1 || check2 {
        return true;
    } else {
        return false;
    } 
}
