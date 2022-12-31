use crate::utils;

#[derive(Eq, PartialEq, Debug)]
enum ListComparison {
    LeftFirst,
    Equal,
    RightFirst,
}

#[derive(Eq, PartialEq)]
enum TokenType {
    ListStart,
    ListEnd,
    Num,
    
}

fn char_to_type(c: char) -> TokenType {
    if c.is_digit(10) {
        return TokenType::Num;
    } else if c == '[' {
        return TokenType::ListStart;
    } else if c == ']' {
        return TokenType::ListEnd;
    } else {
        panic!("invalid token type: {}", c)
    }
}

fn get_int_at_pos(char_vec: &[char], mut char_pos: usize) -> (u32, usize) {
    let mut num_str: String = "".to_string();
    while char_pos < char_vec.len() && char_vec[char_pos].is_digit(10) {
        num_str.push(char_vec[char_pos]);
        char_pos += 1;
    }
    if char_pos < char_vec.len() && char_vec[char_pos] == ',' {
        char_pos += 1;
    }
    let num: u32 = match num_str.parse::<u32>() {
        Ok(v) => v,
        Err(_) => {
            println!("Warning, couldn't get int at pos!");
            0
        },
    };

    return (num, char_pos);
}

fn find_end_of_num(char_vec: &[char], mut char_pos: usize) -> usize {
    while char_pos < char_vec.len() && char_vec[char_pos].is_digit(10) {
        char_pos += 1;
    }

    return char_pos;
}

fn find_end_of_list(char_vec: &[char], mut char_pos: usize) -> usize {
    if char_vec[char_pos] != '[' {
        println!("WARNING: not in a list");
    }
    char_pos += 1;

    let mut num_sub_lists: u32 = 0;
    while char_vec[char_pos] != ']' || num_sub_lists > 0 {
        if char_vec[char_pos] == '[' {
            num_sub_lists += 1;
        } else if char_vec[char_pos] == ']' {
            num_sub_lists -= 1;
        }
        char_pos += 1;
    }

    return char_pos;
}

fn compare_lists(left_chars: &[char], right_chars: &[char]) -> ListComparison {
    let mut left_idx: usize = 0;
    let mut right_idx: usize = 0;

    println!("checking {} vs {}", left_chars.clone().iter().collect::<String>(), right_chars.clone().iter().collect::<String>());

    while left_idx < left_chars.len() && right_idx < right_chars.len() {
        let left_type = char_to_type(left_chars[left_idx]);
        let right_type = char_to_type(right_chars[right_idx]);

        if left_type == TokenType::Num && right_type == TokenType::Num {
            // both nums
            let (left_num, left_pos) = get_int_at_pos(&left_chars, left_idx);
            left_idx = left_pos;

            let (right_num, right_pos) = get_int_at_pos(&right_chars, right_idx);
            right_idx = right_pos;

            if left_num < right_num {
                return ListComparison::LeftFirst;
            } else if left_num > right_num {
                return ListComparison::RightFirst;
            }
            // no else, as we just want to continue the loop in that case

        } else if left_type == TokenType::ListStart && right_type == TokenType::ListStart {
            // both lists, so compare their sub-lists
            let left_stop = find_end_of_list(left_chars, left_idx);
            let right_stop = find_end_of_list(right_chars, right_idx);

            let sub_cmp = compare_lists(&left_chars[(left_idx+1)..left_stop], &right_chars[(right_idx+1)..right_stop]);
            
            // advance past this sub-list
            left_idx += left_stop + 1;
            if left_idx < left_chars.len() && left_chars[left_idx] == ',' {
                left_idx += 1;
            }
            right_idx += right_stop + 1;
            if right_idx < right_chars.len() && right_chars[right_idx] == ',' {
                right_idx += 1;
            }
        
            if sub_cmp != ListComparison::Equal {
                return sub_cmp;
            }
            // no else, as we just want to continue the loop in that case

        } else if left_type == TokenType::ListEnd || right_type == TokenType::ListEnd {
            panic!("shouldn't find list end in this algo");
        } else {
            // neither is ListEnd, one is num and one is ListStart
            let sub_cmp = if left_type == TokenType::Num {
                // only left is num
                let left_stop = find_end_of_num(left_chars, left_idx);
                let right_stop = find_end_of_list(right_chars, right_idx);

                let sub_cmp = compare_lists(&left_chars[left_idx..left_stop], &right_chars[(right_idx+1)..right_stop]);
                right_idx += right_stop + 1;
                if right_idx < right_chars.len() && right_chars[right_idx] == ',' {
                    right_idx += 1;
                }
                left_idx += left_stop + 1;

                sub_cmp
            } else {
                // only right is num
                let left_stop = find_end_of_list(left_chars, left_idx);
                let right_stop = find_end_of_num(right_chars, right_idx);

                let sub_cmp = compare_lists(&left_chars[(left_idx+1)..left_stop], &right_chars[right_idx..right_stop]);
                
                left_idx += left_stop + 1;
                if left_idx < left_chars.len() && left_chars[left_idx] == ',' {
                    left_idx += 1;
                }
                right_idx += right_stop + 1;
                
                sub_cmp
            };
            
            if sub_cmp != ListComparison::Equal {
                return sub_cmp;
            }
        }
    }

    if left_idx == left_chars.len() && right_idx == right_chars.len() {
        return ListComparison::Equal;
    } else if left_idx == left_chars.len() {
        return ListComparison::LeftFirst;
    } else {
        return ListComparison::RightFirst;
    }
    
}

pub fn day13(fname: &str) {
    if let Ok(f) = utils::read_lines(fname) {
    
        let mut line_num: u32 = 0;

        let mut right_idxs: u32 = 0;
        let mut pair_idx: u32 = 1;

        let mut first_pkt: Vec<char> = Vec::new();
        let mut second_pkt: Vec<char> = Vec::new();

        for l in f {
            if let Ok(line) = l {
                // first packet
                if line_num % 3 == 0 {
                    first_pkt = line.chars().collect();
                }

                // second packet
                if line_num % 3 == 1 {
                    second_pkt = line.chars().collect();

                    // comparison
                    let comparison = compare_lists(&first_pkt[..], &second_pkt[..]);
                    if comparison == ListComparison::LeftFirst {
                        right_idxs += pair_idx;
                    } else if comparison == ListComparison::Equal {
                        print!("WARNING - ");
                    }
                    println!("{} - {} - {:?}", line_num, pair_idx, comparison);
                    pair_idx += 1;
                }

                if line_num % 3 == 2 {
                    if line != "" {
                        println!("expected empty line, got {}", &line);
                    }
                }

                line_num += 1;
            }
        }

        println!("sum of right-ordered packets: {}", right_idxs);
    }
}