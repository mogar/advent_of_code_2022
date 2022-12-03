use crate::utils;

pub fn day1(fname: &str) -> () {
    // vector of elves, starts with one elf with 0 cals
    let mut elf_cals: Vec<i32> = vec![0];
    let mut elf: usize = 0;

    if let Ok(calorie_lines) = utils::read_lines(fname) {
        for line in calorie_lines {
            if let Ok(calories) = line {
                if &calories == "" {
                    // start a new elf
                    elf_cals.push(0);
                    elf += 1;
                } else {
                    // add to current elf
                    if let Ok(cals_int) = calories.parse::<i32>() {
                        elf_cals[elf] += cals_int;
                    }
                }
            }
        }
    }

    // find max calories by elf
    if let Some(max_elf) = elf_cals.iter().max() {
        println!("max elf: {}", max_elf);

        let mut top_3_cals: i32 = *max_elf;
        let top_idx = elf_cals.iter().position(|&x| x == *max_elf).unwrap();
        elf_cals.remove(top_idx);

        // second highest
        let top_val = *elf_cals.iter().max().unwrap();
        top_3_cals += top_val;
        let top_idx = elf_cals.iter().position(|&x| x == top_val).unwrap();
        elf_cals.remove(top_idx);

        // third highest
        let top_val = *elf_cals.iter().max().unwrap();
        top_3_cals += top_val;
        let top_idx = elf_cals.iter().position(|&x| x == top_val).unwrap();
        elf_cals.remove(top_idx);

        println!("top three elves carry {} calories", top_3_cals);
    }
}