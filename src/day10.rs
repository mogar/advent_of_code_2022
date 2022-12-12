use crate::utils;
use std::collections::HashSet;

enum Instr {
    Noop,
    Addx,
}

fn parse_instr_type(instr_str: &str) -> Instr {
    match instr_str {
        "noop" => Instr::Noop,
        "addx" => Instr::Addx,
        _ => panic!("invalid instruction type"),
    }
}

fn instr_length(instr_type: &Instr) -> u32 {
    match instr_type {
        Instr::Noop => 1,
        Instr::Addx => 2,
    }
}

fn sig_strength(cyc_num: u32, reg_x: i32) -> i32 {
    (cyc_num as i32) * reg_x
}

fn char_to_draw(cyc_num: u32, reg_x: i32) -> char {
    let cyc = cyc_num % 40;
    let x_dif = reg_x - (cyc as i32);
    if x_dif.abs() <= 1 {
        return '#';
    }
    return '.';
}

pub fn day10(fname: &str) {
    if let Ok(f) = utils::read_lines(fname) {
        let mut reg_x: i32 = 1;
        let mut cycle_count: u32 = 1; // start on first cycle
        let mut sig_accum: i32 = 0;
        let check_set: HashSet<u32> = vec![20, 60, 100, 140, 180, 220].into_iter().collect();

        let mut crt: Vec<Vec<char>> = Vec::new();
        for y in 0..6 {
            crt.push(Vec::new());
            for _ in 0..40 {
                crt[y].push('.');
            }
        }

        for l in f {
            if let Ok(line) = l {
                let instr: Vec<&str> = line.split(" ").collect();

                let instr_type = parse_instr_type(instr[0]);
                let instr_dur = instr_length(&instr_type);
                for _ in 0..instr_dur {
                    // check sig strength
                    if check_set.contains(&cycle_count) {
                        sig_accum += sig_strength(cycle_count, reg_x);
                    }
                    
                    // draw CRT
                    let crt_pos = cycle_count - 1;
                    let y_idx = (crt_pos / 40) % 6;
                    let x_idx = crt_pos % 40;
                    crt[y_idx as usize][x_idx as usize] = char_to_draw(crt_pos, reg_x);
                    
                    // handle cycle_count
                    cycle_count += 1;
                }
                match instr_type {
                    Instr::Noop => {/* do nothing */ },
                    Instr::Addx => {
                        let reg_delta = instr[1].parse::<i32>().unwrap();
                        reg_x += reg_delta;
                    },
                }
            }
        }

        // fencepost for last instr
        if check_set.contains(&cycle_count) {
            sig_accum += sig_strength(cycle_count, reg_x);
        }

        println!("total accumulated signal: {}", sig_accum);
        for y_idx in 0..crt.len() {
            for x_idx in 0..crt[y_idx].len() {
                print!("{}", crt[y_idx][x_idx]);
            } 
            println!("");
        }
    }
}