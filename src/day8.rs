use crate::utils;

fn is_visible(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> (bool, u32) {
    let mut real_viz = false;
    if x == 0 || y == 0 || x == (forest.len() - 1) || y == (forest[0].len() - 1) {
        real_viz = true;
    }

    let our_height: u32 = forest[x][y];

    // north
    let mut viz: bool = true; // assume at first
    let mut north_view: u32 = 0;
    for idx in (0..x).rev() {
        north_view += 1;
        if forest[idx][y] >= our_height {
            viz = false;
            break;
        }
    }
    if viz {
        real_viz = true;
    }

    // south
    viz = true;
    let mut south_view: u32 = 0;
    for idx in (x+1)..forest.len() {
        south_view += 1;
        if forest[idx][y] >= our_height {
            viz = false;
            break;
        }
    }
    if viz {
        real_viz = true;
    }

    // east
    viz = true;
    let mut east_view: u32 = 0;
    for idx in (0..y).rev() {
        east_view += 1;
        if forest[x][idx] >= our_height {
            viz = false;
            break;
        }
    }
    if viz {
        real_viz = true;
    }

    // west
    viz = true;
    let mut west_view: u32 = 0;
    for idx in (y+1)..forest[0].len() {
        west_view += 1;
        if forest[x][idx] >= our_height {
            viz = false;
            break;
        }
    }
    if viz {
        real_viz = true;
    }

    let scenic_score = north_view * south_view * east_view * west_view;

    return (real_viz, scenic_score);
}

pub fn day8(fname: &str) -> () {
    if let Ok(tree_lines) = utils::read_lines(fname) {
        let mut forest: Vec<Vec<u32>> = Vec::new();
        for tree_line in tree_lines {
            if let Ok(trees) = tree_line {
                forest.push(trees.chars().map(|c| c as u32).collect());
            }
        }

        let y_max: u32 = forest[0].len() as u32;
        println!("forest size is {} by {}", forest.len(), forest[0].len());
        let mut num_visible: usize = 0;
        let mut max_score: u32 = 0;
        for x in 0..forest.len() {
            for y in 0..forest[0].len() {
                let (visible, score) = is_visible(&forest, x, y);
                if visible {
                    num_visible += 1;
                }
                if score > max_score {
                    max_score = score;
                }
            }
        }
        println!("{} visible trees", num_visible);
        println!("max scenic score: {}", max_score);
    }

}