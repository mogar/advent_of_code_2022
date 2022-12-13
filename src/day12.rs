use crate::utils;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl Eq for Point {}


fn update_checkables(topo: &Vec<Vec<u32>>, 
                    reach_cost: &Vec<Vec<u32>>, 
                    cur_point: &Point, 
                    next_point: &Point, 
                    checkables: &mut HashMap<Point, u32>) {
    if (topo[cur_point.y][cur_point.x] + 1) >= topo[next_point.y][next_point.x] {
        if reach_cost[next_point.y][next_point.x] == u32::MAX {
            let cost = reach_cost[cur_point.y][cur_point.x] + 1;

            if checkables.get(next_point).unwrap_or(&u32::MAX) > &cost {
                checkables.insert(*next_point, cost);
            }
        }
    }
}

fn get_next_point(checkables: &mut HashMap<Point, u32>) -> (Point, u32) {
    let mut next_point = Point {x: 0, y: 0};
    let mut next_cost: u32 = u32::MAX;
    for (p, c) in checkables.iter() {
        if *c < next_cost {
            next_cost = *c;
            next_point = *p;
        }
    }

    checkables.remove(&next_point);
    return (next_point, next_cost);
}

pub fn day12(fname: &str) {
    if let Ok(f) = utils::read_lines(fname) {
        let mut topo: Vec<Vec<u32>> = Vec::new();
        let mut reach_cost: Vec<Vec<u32>> = Vec::new();
        let mut h: usize = 0;

        // Note that we start the "start" vec with a dummy that will hold our initial starter
        let mut starts: Vec<Point> = vec![Point{x: 0, y: 0}];
        let mut end = Point{x: 0, y: 0};

        for l in f {
            if let Ok(line) = l {
                topo.push(Vec::new());
                reach_cost.push(Vec::new());
                for c in line.chars() {
                    if c == 'S' {
                        // current pos 'S' has elev a
                        starts[0].x = topo[h].len();
                        starts[0].y = h;
                        topo[h].push('a' as u32);
                    } else if c == 'E' {
                        // best signal 'E' has elev z
                        end.x = topo[h].len();
                        end.y = h;
                        topo[h].push('z' as u32);
                    } else {
                        if c == 'a' {
                            starts.push(Point{x: topo[h].len(), y: h});
                        }
                        topo[h].push(c as u32);
                    }
                    reach_cost[h].push(u32::MAX);
                }
                h += 1;
            }
        }

        let w: usize = topo[0].len();

        println!("size: {} by {}", w, h);

        // can only move cardinal dirs
        // can step only one elev up or down

        let mut first_run: bool = true;
        let mut lowest_costs: Vec<u32> = Vec::new();
        for start in starts {
            // reset for next run
            for y in 0..reach_cost.len() {
                for x in 0..reach_cost[y].len() {
                    reach_cost[y][x] = u32::MAX;
                }
            }
            
            let mut checkable_points: HashMap<Point, u32> = HashMap::new();
            checkable_points.insert(start, 0);


            while reach_cost[end.y][end.x] == u32::MAX {
                let (cur_point, cur_cost) = get_next_point(&mut checkable_points);
                if cur_cost == u32::MAX {
                    //no good path to E from start
                    break;
                }
                reach_cost[cur_point.y][cur_point.x] = cur_cost;

                if cur_point.y > 0 {
                    // can check going North
                    let next_point = Point{x: cur_point.x, y: cur_point.y - 1};
                    update_checkables(&topo, &reach_cost, &cur_point, &next_point, &mut checkable_points);
                }

                if cur_point.y < (h-1) {
                    // check going South
                    let next_point = Point{x: cur_point.x, y: cur_point.y + 1};
                    update_checkables(&topo, &reach_cost, &cur_point, &next_point, &mut checkable_points);
                }

                if cur_point.x > 0 {
                    // check going West
                    let next_point = Point{x: cur_point.x - 1, y: cur_point.y};
                    update_checkables(&topo, &reach_cost, &cur_point, &next_point, &mut checkable_points);
                }

                if cur_point.x < (w-1) {
                    // check going East
                    let next_point = Point{x: cur_point.x + 1, y: cur_point.y};
                    update_checkables(&topo, &reach_cost, &cur_point, &next_point, &mut checkable_points);
                }
            }

            if reach_cost[end.y][end.x] < u32::MAX {
                lowest_costs.push(reach_cost[end.y][end.x]);
            }

            if first_run {
                first_run = false;
                println!("min travel cost from initial start: {}", reach_cost[end.y][end.x]);
            }
        }

        lowest_costs.sort_by(|a, b| b.cmp(&a));
        println!("overall lowest {} {}", lowest_costs.pop().unwrap(), lowest_costs.pop().unwrap());
    }
}