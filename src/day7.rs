use crate::utils;
use std::collections::HashMap;

fn update_dir_path(dir_path: &mut Vec<String>, new_dir: &str) -> () {
    if new_dir == ".." {
        dir_path.pop();
    } else if new_dir == "/" {
        dir_path.clear();
    } else {
        dir_path.push(new_dir.to_string());
    }
}

fn add_to_sizes(dir_sizes: &mut HashMap<String, usize>, entry: &str, size: usize) {
    if dir_sizes.contains_key(entry) {
        let new_size: usize = dir_sizes[entry];
        dir_sizes.insert(entry.to_string(), new_size + size);
    } else {
        dir_sizes.insert(entry.to_string(), size);
    }
}

pub fn day7(fname: &str) -> () {
    if let Ok(nav_lines) = utils::read_lines(fname) {
        let mut dir_sizes: HashMap<String, usize> = HashMap::new();
        let mut dir_path: Vec<String> = Vec::new();

        for nav_val in nav_lines {
            if let Ok(nav) = nav_val {
                let nav_parts: Vec<&str> = nav.split_whitespace().collect();
                if nav_parts[0].parse::<usize>().is_ok() {
                    let file_size: usize = nav_parts[0].parse::<usize>().unwrap();
                    let mut dir_entry = "/".to_owned();
                    
                    for dir in &dir_path {
                        add_to_sizes(&mut dir_sizes, &dir_entry, file_size);
                        dir_entry.push_str(&dir);
                        dir_entry.push_str("/");
                    }
                    // fencepost for final dir entry
                    add_to_sizes(&mut dir_sizes, &dir_entry, file_size);
                } else if nav_parts[0] == "$" && nav_parts[1] == "cd" {
                    update_dir_path(&mut dir_path, nav_parts[2]);
                }
            }
        }

        let small_dirs: Vec<usize> = dir_sizes.values().filter(|s| s <= &&100000).cloned().collect();
        let small_dir_total: usize = small_dirs.iter().sum();
        println!("sum of small directories: {}", small_dir_total);

        println!("current free space: {}", dir_sizes["/"]);

        let needed: usize = 30000000;
        let remainder: usize = 70000000 - dir_sizes["/"];
        let min_delete: usize = needed - remainder;
        let mut possible_dels: Vec<(&String, &usize)> = dir_sizes.iter().filter(|(_, s)| s >= &&min_delete).collect();
        possible_dels.sort_by(|(_, a), (_, b)| a.cmp(b));
        let (del_name, del_size) = possible_dels[0];
        println!("delete {} to gain {} back", del_name, del_size);
    }
}