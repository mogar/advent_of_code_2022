use crate::utils;
use std::collections::HashSet;

fn get_priority(item: char) -> u32 {
    let val: u32 = if item.is_uppercase() {
        (item as u32) - ('A' as u32) + 27
    } else {
        (item as u32) - ('a' as u32) + 1
    };

    val
}

pub fn day3(fname: &str) -> () {
    if let Ok(packs) = utils::read_lines(fname) {
        let mut prio_sum: u32 = 0;

        let mut badge_set: HashSet<char> = HashSet::new();
        let mut elf_id: u32 = 0;
        let mut badge_sum: u32 = 0;

        for line in packs {
            let item_string = line.unwrap();
            let mut items = item_string.chars();

            let first_half: HashSet<char> = items.by_ref().take(item_string.len()/2).collect();
            // second half is just hashset of what's left after taking the first half
            let second_half: HashSet<char> = items.by_ref().collect();
            let mut common = first_half.intersection(&second_half);

            // we're given that there's only one char in the intersection

            let priority = get_priority(*common.next().unwrap());
            prio_sum += priority;

            if elf_id == 0 {
                badge_set = item_string.chars().collect();
            } else {
                let temp_set: HashSet<char> = item_string.chars().collect();
                badge_set = badge_set.intersection(&temp_set).cloned().collect();
                if elf_id == 2 {
                    if badge_set.len() == 1 {
                        let mut badge_iter = badge_set.drain();
                        let badge_prio: u32 = get_priority(badge_iter.next().unwrap());
                        badge_sum += badge_prio;
                    } else {
                        println!("Problem with badge priorities");
                    }
                }
            }

            elf_id = (elf_id + 1) % 3;
        }
        println!("total sum of priorities: {}", prio_sum);
        println!("total sum of badge priorities: {}", badge_sum);
    }
}
