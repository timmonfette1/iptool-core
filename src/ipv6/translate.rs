/* iptool_core
 Rust program to do IP related tasks.

 Functions to convert IPv6 to IPv4.

 Author: Tim Monfette
 Version: 1.0.0
*/

use utilities::helpers::str_to_int;
use super::validate::validate_6to4;

// Function for IPv6 to IPv4
pub fn ipv6_to_ipv4(v6: String) -> String {
    let valid = validate_6to4(&v6);
    if valid == false {
        let err = "IPv6 address is not valid! Cannot translate this address\nValid IPv6 to IPv4 addresses are:\n2002::xxxx:xxxx or 2002:xxxx:xxxx::";
        return err.to_owned();
    }

    let split = v6.trim().split(":");
    let mut holder = vec![0, 0, 0, 0];  // vector to hold 8 bit chunks of IPv4

    // Process each 16 bits
    let mut count = 0;
    for s in split {
        if s.len() == 0 || s == "2002" {
            continue;
        }

        // Split octet into 2, 8 bit chunks
        let collected = s.chars().collect::<Vec<_>>();
        let chunks = collected.chunks(2);

        // Process each 8 bits (2 hex values)
        for ch in chunks {
            let pos0 = hex_to_int(ch[0]);
            let pos1 = hex_to_int(ch[1]);

            holder[count] = (pos0 * 16) + (pos1);   // hex to decimal

            count = count + 1;
        }
    }

    let v4 = format!("{}.{}.{}.{}", holder[0], holder[1], holder[2], holder[3]);

    v4
}

// Function to get int value of Hex digit
fn hex_to_int(digit: char) -> i32 {
    let num: i32;

    match digit {
        'a' | 'A' => num = 10,
        'b' | 'B' => num = 11,
        'c' | 'C' => num = 12,
        'd' | 'D' => num = 13,
        'e' | 'E' => num = 14,
        'f' | 'F' => num = 15,
        _         => num = str_to_int(&digit.to_string()),
    }

    num
}
