use crate::utils;

enum BoxParserState {
    StoreBoxLine,
    HandleInstruction,
}

struct BoxInstruction {
    num_boxes: usize,
    origin_stack: usize,
    dest_stack: usize,
}

fn end_of_box_lines(line: &str) -> bool {
    !line.contains('[')
}

fn read_box_locs(box_lines: &Vec<String>, line: &str, box_locs: &mut Vec<Vec<char>>) -> () {
    // figure out how many stacks from line
    let num_stacks: usize = line.split_whitespace().collect::<Vec<&str>>().len();

    // add the number of stacks to box_locs
    for _i in 0..num_stacks {
        box_locs.push(Vec::new());
    }

    // fill all stacks
    for line in box_lines.iter().rev() {
        let mut char_idx: usize = 0;
        for next_char in line.chars() {
            char_idx += 1;
            if next_char.is_alphabetic() {
                let stack_idx = (char_idx - 2)/4;
                box_locs[stack_idx].push(next_char);
            }
        }
    }
}

fn parse_line_instr(line: &str) -> BoxInstruction {
    let nums: Vec<usize> = line.split(" ").filter(|s| s.parse::<usize>().is_ok()).map(|s| s.parse().unwrap()).collect();

    BoxInstruction {
        num_boxes: nums[0],
        origin_stack: nums[1] - 1,
        dest_stack: nums[2] - 1,
    }
}

fn perform_instr_on9000(box_locs: &mut Vec<Vec<char>>, instr: &BoxInstruction) {
    let count: usize = instr.num_boxes;

    for _idx in 0..count {
        let moved_box: char = box_locs[instr.origin_stack].pop().unwrap();
        box_locs[instr.dest_stack].push(moved_box);
    }
}

fn perform_instr_on9001(box_locs: &mut Vec<Vec<char>>, instr: &BoxInstruction) {
    let count: usize = instr.num_boxes;

    let mut temp_vec: Vec<char> = Vec::new();
    for _idx in 0..count {
        let moved_box: char = box_locs[instr.origin_stack].pop().unwrap();
        temp_vec.push(moved_box);
    }

    for _idx in 0..count {
        box_locs[instr.dest_stack].push(temp_vec.pop().unwrap());
    }
}


fn get_top_boxes(box_locs: &Vec<Vec<char>>) -> Vec<char> {
    let mut tops: Vec<char> = Vec::new();
    for stack in box_locs {
        tops.push(stack.last().unwrap().clone());
    }

    tops
}

pub fn day5(fname: &str) -> () {
    let mut box_locs = Vec::new();
    let mut box_locs9001 = Vec::new();
    let mut parser_state = BoxParserState::StoreBoxLine;
    let mut box_lines: Vec<String> = vec!();
    if let Ok(section_assignments) = utils::read_lines(fname) {
        for line in section_assignments {
            let the_line = line.unwrap();
            match parser_state {
                BoxParserState::StoreBoxLine => {
                    if end_of_box_lines(&the_line) {
                        read_box_locs(&box_lines, &the_line, &mut box_locs);
                        box_locs9001 = box_locs.clone();
                        parser_state = BoxParserState::HandleInstruction;
                    } else {
                        box_lines.push(the_line.clone());
                    }
                },
                BoxParserState::HandleInstruction => {
                    if the_line != "" {
                        let instr = parse_line_instr(&the_line);
                        perform_instr_on9000(&mut box_locs, &instr);
                        perform_instr_on9001(&mut box_locs9001, &instr);
                    }
                },
            }
        }

        let top_boxes: String = get_top_boxes(&box_locs).iter().collect();
        println!("top boxes are: {}", top_boxes);

        let top_boxes9001: String = get_top_boxes(&box_locs9001).iter().collect();
        println!("top boxes are: {}", top_boxes9001);
    }
}