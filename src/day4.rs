use crate::utils;

fn range_str_to_vals(range: &str) -> (u32, u32) {
    let vals: Vec<&str> = range.split("-").collect();

    let min: u32 = vals[0].parse::<u32>().unwrap();
    let max: u32 = vals[1].parse::<u32>().unwrap();

    (min, max)
}

pub fn day4(fname: &str) -> () {
    if let Ok(section_assignments) = utils::read_lines(fname) {

        let mut full_overlaps: u32 = 0;
        let mut partial_overlaps: u32 = 0;
        for line in section_assignments {
            // lines are: #-#,#-#; # can be multiple digits long
            let assignment = line.unwrap();
            let elf_assigns: Vec<&str> = assignment.split(",").collect();


            let (e1_min, e1_max) = range_str_to_vals(&elf_assigns[0]);
            let (e2_min, e2_max) = range_str_to_vals(&elf_assigns[1]);

            // how many full overlaps
            if e1_min < e2_min {
                if e2_max <= e1_max {
                    full_overlaps += 1;
                }
            } else if e1_min > e2_min {
                if e1_max <= e2_max {
                    full_overlaps += 1;
                }
            } else {
                // they're equal from the start
                full_overlaps += 1;
            }

            if ((e1_min <= e2_min) && (e1_max >= e2_min)) ||
                ((e1_min >= e2_min) && (e1_min <= e2_max)) {
                    partial_overlaps += 1;
            }
        }
        println!("number of full overlaps: {}", full_overlaps);
        println!("number of partial overlaps: {}", partial_overlaps);
    }
}