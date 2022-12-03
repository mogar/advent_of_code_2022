use crate::utils;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RpsPlay {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RpsResult {
    Win,
    Lose,
    Tie,
}

fn win_lose_tie(oppo_play: RpsPlay, my_play: RpsPlay) -> RpsResult {
    if oppo_play == my_play {
        return RpsResult::Tie;
    }

    if oppo_play == RpsPlay::Rock {
        if my_play == RpsPlay::Paper {
            return RpsResult::Win;
        } else {
            return RpsResult::Lose;
        }
    } else if oppo_play == RpsPlay::Paper {
        if my_play == RpsPlay::Scissors {
            return RpsResult::Win;
        } else {
            return RpsResult::Lose;
        }
    } else { // oppo_play == RpsPlay::Scissors
        if my_play == RpsPlay::Rock {
            return RpsResult::Win;
        } else {
            return RpsResult::Lose;
        }
    }
}

fn score_play(oppo_play: RpsPlay, my_play: RpsPlay) -> u32 {
    let shape_score: u32 = match my_play {
        RpsPlay::Rock => 1,
        RpsPlay::Paper => 2,
        RpsPlay::Scissors => 3,
    };

    let win_score: u32 = match win_lose_tie(oppo_play, my_play) {
        RpsResult::Win => 6,
        RpsResult::Lose => 0,
        RpsResult::Tie => 3,
    };

    shape_score + win_score
}

fn choose_play(oppo_play: RpsPlay, goal: RpsResult) -> RpsPlay {
    if goal == RpsResult::Tie {
        return oppo_play;
    }

    if oppo_play == RpsPlay::Rock {
        if goal == RpsResult::Win {
            return RpsPlay::Paper;
        } else {
            return RpsPlay::Scissors;
        }
    } else if oppo_play == RpsPlay::Paper {
        if goal == RpsResult::Win {
            return RpsPlay::Scissors;
        } else {
            return RpsPlay::Rock;
        }
    } else { // oppo_play == RpsPlay::Scissors
        if goal == RpsResult::Win {
            return RpsPlay::Rock;
        } else {
            return RpsPlay::Paper;
        }
    }
}

pub fn day2(fname: &str) {
    let mut opponent_map: HashMap<&str, RpsPlay> = HashMap::new();
    opponent_map.insert("A", RpsPlay::Rock);
    opponent_map.insert("B", RpsPlay::Paper);
    opponent_map.insert("C", RpsPlay::Scissors);

    let mut self_map: HashMap<&str, RpsPlay> = HashMap::new();
    self_map.insert("X", RpsPlay::Rock);
    self_map.insert("Y", RpsPlay::Paper);
    self_map.insert("Z", RpsPlay::Scissors);

    let mut goal_map: HashMap<&str, RpsResult> = HashMap::new();
    goal_map.insert("X", RpsResult::Lose);
    goal_map.insert("Y", RpsResult::Tie);
    goal_map.insert("Z", RpsResult::Win);

    let mut score: u32 = 0;
    let mut score_actual: u32 = 0;
    if let Ok(rps_strat) = utils::read_lines(fname) {
        for line in rps_strat {
            // split line
            let play_line = line.unwrap();
            let plays: Vec<&str> = play_line.split(" ").collect();

            let oppo = opponent_map[&plays[0]];
            let me_guess = self_map[&plays[1]];
            let me_actual = choose_play(oppo, goal_map[&plays[1]]);

            // get points
            score += score_play(oppo, me_guess);
            score_actual += score_play(oppo, me_actual);
        }
    }

    println!("score: {}", score);
    println!("score actual: {}", score_actual);
}