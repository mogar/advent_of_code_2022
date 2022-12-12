use crate::utils;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct EndLoc {
    x: i32,
    y: i32,
}

fn move_head(head: &mut EndLoc, dir: char) -> () {
    if dir == 'R' {
        head.x += 1;
    } else if dir == 'L' {
        head.x -= 1;
    } else if dir == 'U' {
        head.y += 1;
    } else if dir == 'D' {
        head.y -= 1;
    }
}

fn drag_tail(head: EndLoc, tail: &mut EndLoc) -> () {
    let x_dif: i32 = head.x - tail.x;
    let y_dif: i32 = head.y - tail.y;

    if !((x_dif.abs() <= 1) && (y_dif.abs() <= 1)) {
        tail.x += x_dif.clamp(-1, 1);
        tail.y += y_dif.clamp(-1, 1);
    }
}

pub fn day9(fname: &str) -> () {
    if let Ok(f) = utils::read_lines(fname) {
        // start at the same pos
        let num_knots: usize = 10;
        let mut knots: Vec<EndLoc> = Vec::new();
        for _ in 0..num_knots {
            knots.push(EndLoc {x: 0, y: 0});
        }
        let last_tail = knots.len() - 1;
        println!("last tail: {}", last_tail);

        let mut tail1_visited: HashSet<EndLoc> = HashSet::new();
        let mut tail9_visited: HashSet<EndLoc> = HashSet::new();
        tail1_visited.insert(knots[1].clone());
        tail9_visited.insert(knots[last_tail].clone());

        for l in f {
            if let Ok(line) = l {
                let instr: Vec<&str> = line.split(" ").collect();
                let stride = instr[1].parse::<usize>().unwrap();
                let dir: char = instr[0].chars().next().unwrap();

                for _ in 0..stride {
                    move_head(&mut knots[0], dir);
                    for idx in 1..num_knots {
                        let leader = knots[idx-1];
                        drag_tail(leader, &mut knots[idx]);
                    }
                    tail1_visited.insert(knots[1]);
                    tail9_visited.insert(knots[last_tail]);                }
            }
        }

        println!("tail 1 visited {} locations", tail1_visited.len());
        println!("tail 9 visited {} locations", tail9_visited.len());
    }
}
