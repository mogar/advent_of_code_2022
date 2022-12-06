use crate::utils;
use std::collections::VecDeque;
use std::collections::HashSet;

fn add_sym_to_marker(marker: &mut VecDeque<char>, marker_size: usize, sym: char) {
    marker.push_back(sym);
    if marker.len() > marker_size {
        marker.pop_front();
    }
}

fn marker_is_valid(marker: &VecDeque<char>, marker_size: usize) -> bool {
    if marker.len() == marker_size {
        let set: HashSet<char> = marker.iter().cloned().collect();
        if set.len() == marker_size {
            return true;
        }
    }
    return false;
}

pub fn day6(fname: &str) -> () {
    if let Ok(code_input) = utils::read_lines(fname) {
        for line in code_input {
            // detect start of packet (four different chars)
            let line_u = line.unwrap();
            let chars = line_u.chars();

            let mut found_marker: bool = false;
            let mut found_message: bool = false;

            let mut marker: VecDeque<char> = VecDeque::new();
            let mut message: VecDeque<char> = VecDeque::new();
            let mut sym_count: u32 = 0;
            for s in chars {
                add_sym_to_marker(&mut marker, 4, s);
                add_sym_to_marker(&mut message, 14, s);
                sym_count += 1;
                if !found_marker && marker_is_valid(&marker, 4) {
                    println!("marker found at {}", sym_count);
                    found_marker = true;
                }
                if !found_message && marker_is_valid(&message, 14) {
                    println!("message found at {}", sym_count);
                    found_message = true;
                }
                if found_message && found_marker {
                    break;
                }
            }

        }
    }
}