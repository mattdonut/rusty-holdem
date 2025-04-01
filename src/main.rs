pub mod deck;
pub mod game;
use crate::game::Game;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let player_count = args[1].parse::<usize>().unwrap();
    let hand_count = args[2].parse::<usize>().unwrap();

    println!("Lets Play!");
    let mut hands_played = 0;
    let mut real_ties = 0;
    let mut winning_ties = 0;
    let mut game = Game::new(player_count);
    for _ in 0..hand_count {
        let maybe_result = game.play_hand();
        match maybe_result {
            Some(result) => match [result.nondegenerate_tie, result.winning_tie] {
                [Some(tie), Some(push)] => {
                    hands_played += 1;
                    if tie {
                        real_ties += 1;
                    }
                    if push {
                        winning_ties += 1;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
    println!(
        "Results!! {:?} Ties, {:?} pushes out of {:?} Hands",
        real_ties, winning_ties, hands_played
    );
}
