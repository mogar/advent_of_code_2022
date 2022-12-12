use crate::utils;

enum MonkeyParseState {
    Id,
    Items,
    Op,
    Test,
    True,
    False,
}

#[derive(Copy, Clone, Debug)]
struct MonkeyOp {
    op_type: char,
    op_old_n_val: bool,
    op_val: u64,
}

impl MonkeyOp {
    fn new() -> Self {
        Self {
            op_type: ' ',
            op_old_n_val: false,
            op_val: 0,
        }
    }
}

#[derive(Copy, Clone)]
struct Monkey {
    id: usize,
    op: MonkeyOp,
    test: u64,
    pass: usize,
    fail: usize,
    inspected: u64,
}

impl Monkey {
    fn new() -> Self {
        Self {
            id: 0,
            op: MonkeyOp::new(),
            test: 1,
            pass: 0,
            fail: 0,
            inspected: 0,
        }
    }
}

fn monkey_op(monkey: &Monkey, item: u64) -> u64 {
    // return updated item worry
    match monkey.op.op_type {
        '+' => {
            if monkey.op.op_old_n_val {
                return item + item;
            } else {
                return item + monkey.op.op_val;
            }
        },
        '*' => {
            if monkey.op.op_old_n_val {
                return item * item;
            } else {
                return item * monkey.op.op_val;
            }

        },
        _ => panic!("invalid op!"),
    }
}

fn monkey_test(monkey: &Monkey, item: u64) -> bool {
    (item % monkey.test) == 0
}

pub fn day11(fname: &str) {
    if let Ok(f) = utils::read_lines(fname) {
        let mut monkeys: Vec<Monkey> = Vec::new();
        let mut monkey_items: Vec<Vec<u64>> = Vec::new();
        
        let mut parse_state = MonkeyParseState::Id;
        let mut monkey_num: usize = 0;
        for l in f {
            if let Ok(line) = l {
                match parse_state {
                    MonkeyParseState::Id => {
                        let parts: Vec<&str> = line.split(" ").collect();
                        if parts.len() > 0 && parts[0] == "Monkey" {
                            let num_str: Vec<&str> = parts[1].split(":").collect();
                            monkey_num = num_str[0].parse::<usize>().unwrap();
                            monkeys.push(Monkey::new());
                            monkeys[monkey_num].id = monkey_num;
                            monkey_items.push(Vec::new());
                            parse_state = MonkeyParseState::Items;
                        }
                    },
                    MonkeyParseState::Items => {
                        let parts: Vec<&str> = line.split(":").collect();
                        let nums: Vec<&str> = parts[1].split(",").collect();
                        for i in nums.iter() {
                            monkey_items[monkey_num].push(i.trim().parse::<u64>().unwrap());
                        }
                        parse_state = MonkeyParseState::Op;
                    },
                    MonkeyParseState::Op => {
                        let parts: Vec<&str> = line.split("=").collect();
                        let eqn_parts: Vec<&str> = parts[1].trim().split(" ").collect();
                        let op_type = eqn_parts[1].chars().next().unwrap();
                        let op_old_n_val = eqn_parts[2] == "old";
                        let op_val = if op_old_n_val {
                            0
                        } else {
                            eqn_parts[2].parse::<u64>().unwrap()
                        };
                        let op = MonkeyOp{op_type: op_type, op_old_n_val: op_old_n_val, op_val: op_val,};
                        monkeys[monkey_num].op = op;
                        parse_state = MonkeyParseState::Test;
                    },
                    MonkeyParseState::Test => {
                        let parts: Vec<&str> = line.split(" ").collect();
                        let div_idx = parts.len() - 1;
                        monkeys[monkey_num].test = parts[div_idx].parse::<u64>().unwrap();
                        parse_state = MonkeyParseState::True;
                    },
                    MonkeyParseState::True => {
                        let parts: Vec<&str> = line.split(" ").collect();
                        let to_idx = parts.len() - 1;
                        monkeys[monkey_num].pass = parts[to_idx].parse::<usize>().unwrap();
                        parse_state = MonkeyParseState::False;
                    },
                    MonkeyParseState::False => {
                        let parts: Vec<&str> = line.split(" ").collect();
                        let to_idx = parts.len() - 1;
                        monkeys[monkey_num].fail = parts[to_idx].parse::<usize>().unwrap();
                        parse_state = MonkeyParseState::Id;
                    },
                }
                
            }
        }

        let mut div_test_product: u64 = 1;
        for m in &monkeys {
            div_test_product *= m.test;
        }

        let num_rounds = 10000; // was 20 in ph1
        for _ in 0..num_rounds {
            for m in 0..monkeys.len() {
                let mut monkey = monkeys[m];
                let items: Vec<u64> = monkey_items[m].drain(..).collect();
                for i in items {
                    monkey.inspected += 1;
                    // note div by 3 for phase 1 only
                    let new_item = monkey_op(&monkey, i) % div_test_product; // /3;

                    let new_monkey = if monkey_test(&monkey, new_item) {
                        monkey.pass
                    } else {
                        monkey.fail
                    };
                    monkey_items[new_monkey].push(new_item);
                }
                monkeys[m] = monkey;
            }
        }
    
        monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
        let monkey_business = monkeys[0].inspected * monkeys[1].inspected;
        println!("monkey business after 20 steps: {}", monkey_business);
    
    
    }
}