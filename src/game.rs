use crate::deck::count_ranks;
use crate::deck::get_count_ranks;
use crate::deck::Card;
use crate::deck::Deck;
use crate::deck::Rank;
use crate::deck::Suit;
use std::fmt;

#[derive(Debug)]
pub struct Game {
    deck: Deck,
    player_count: usize,
}

#[derive(Debug, Clone)]
pub struct Player {
    hand: Option<HandValue>,
    winner: Option<bool>,
    degenerate: Option<bool>,
    tied: Option<bool>,
}

#[derive(Debug)]
pub struct HandResult {
    common_hand: Option<HandValue>,
    winning_hand: Option<HandValue>,
    pub winning_tie: Option<bool>,
    pub nondegenerate_tie: Option<bool>,
}

impl fmt::Display for HandResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Common: {:?}, Winning: {:?}, Tie: {:?}, Push: {:?}",
            self.common_hand, self.winning_hand, self.nondegenerate_tie, self.winning_tie
        )
    }
}

impl Player {
    pub fn set_tie(&mut self, did_tie: bool) {
        self.tied = Some(did_tie);
    }
    pub fn set_winner(&mut self, did_win: bool) {
        self.winner = Some(did_win);
    }
    pub fn set_degenerate(&mut self, is_degenerate: bool) {
        self.degenerate = Some(is_degenerate);
    }
}

impl Game {
    pub fn new(player_count: usize) -> Game {
        return Game {
            deck: Deck::new(),
            player_count,
        };
    }
    pub fn play_hand(&mut self) -> Option<HandResult> {
        self.deck.shuffle();
        let mut players: Vec<Player> = vec![];
        if let Some((shared, mut left)) = self.deck.cards[..].split_first_chunk::<5>() {
            let common_hand = shared.iter().cloned().collect::<Vec<Card>>();
            for _ in 0..self.player_count {
                if let Some((mine, more_left)) = left.split_first_chunk::<2>() {
                    let mut my_hand: Vec<Card> = vec![];
                    my_hand.clone_from(&mine.iter().chain(shared).cloned().collect::<Vec<Card>>());
                    my_hand.sort();
                    left = more_left;
                    players.push(Player {
                        hand: get_best_hand(&my_hand),
                        winner: None,
                        degenerate: None,
                        tied: None,
                    });
                }
            }
            let winning_hand = players.iter().filter_map(|player| player.hand).max();
            let common_hand = get_best_hand(&common_hand);
            let all_hands = players
                .iter()
                .filter_map(|player| player.hand)
                .clone()
                .collect::<Vec<HandValue>>();
            let tie_hands = all_hands[..all_hands.len() - 1]
                .iter()
                .enumerate()
                .filter_map(|(index, hand)| {
                    if all_hands[index + 1..].contains(hand) {
                        Some(*hand)
                    } else {
                        None
                    }
                })
                .collect::<Vec<HandValue>>();

            for player in players.iter_mut() {
                match player.hand {
                    Some(hand) => {
                        player.set_tie(tie_hands.iter().as_slice().contains(&hand));
                        player.set_winner(Some(hand) == winning_hand);
                        player.set_degenerate(Some(hand) == common_hand);
                    }
                    None => player.set_tie(false),
                }
            }
            let winning_tie = Some(tie_hands.iter().as_slice().contains(&winning_hand.unwrap()));
            let nondegenerate_tie = Some(
                tie_hands
                    .iter()
                    .cloned()
                    .filter(|hand| hand != &common_hand.unwrap())
                    .count()
                    > 0,
            );

            return Some(HandResult {
                common_hand,
                winning_hand,
                winning_tie,
                nondegenerate_tie,
            });
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandValue {
    HighCard(Rank),
    Pair(Rank),
    TwoPair(Rank, Rank),
    Trip(Rank),
    Straight(Rank),
    Flush(Rank, Rank, Rank, Rank, Rank),
    FullHouse(Rank, Rank),
    Quad(Rank),
    StraightFlush(Rank),
}

fn get_best_multiple(cards: &[Card]) -> Option<HandValue> {
    let counts = get_count_ranks(count_ranks(cards));
    match counts[4].last() {
        Some(rank) => Some(HandValue::Quad(*rank)),
        None => match counts[3].last() {
            Some(rank) => match counts[3].get(counts[3].len().wrapping_sub(2)) {
                Some(second_trip) => Some(HandValue::FullHouse(*rank, *second_trip)),
                None => match counts[2].last() {
                    Some(pair_rank) => Some(HandValue::FullHouse(*rank, *pair_rank)),
                    None => Some(HandValue::Trip(*rank)),
                },
            },
            None => match counts[2].last() {
                Some(first_pair) => match counts[2].get(counts[2].len().wrapping_sub(2)) {
                    Some(second_pair) => Some(HandValue::TwoPair(*first_pair, *second_pair)),
                    None => Some(HandValue::Pair(*first_pair)),
                },
                None => match counts[1].last() {
                    Some(rank) => Some(HandValue::HighCard(*rank)),
                    None => None,
                },
            },
        },
    }
}

fn get_straight(cards: &[Card]) -> Option<HandValue> {
    if cards.len() < 5 {
        return None;
    }
    let ranks: Vec<Rank> = cards.iter().map(|card| card.0).collect();
    // if we have an Ace, count it as the starting point of a low straight
    let mut consec = if ranks.contains(&Rank::A) { 1 } else { 0 };
    let mut max: Option<HandValue> = None;
    for rank in Rank::iterator() {
        if ranks.contains(&rank) {
            consec += 1;
            if consec > 4 {
                max = Some(HandValue::Straight(rank));
            }
        } else {
            consec = 0;
        }
    }
    return max;
}

fn split_suits(cards: &[Card]) -> [Vec<Card>; 4] {
    let spades: Vec<Card> = cards
        .iter()
        .filter(|card| card.1 == Suit::Spade)
        .cloned()
        .collect::<Vec<Card>>();
    let hearts: Vec<Card> = cards
        .iter()
        .filter(|card| card.1 == Suit::Heart)
        .cloned()
        .collect::<Vec<Card>>();
    let clubs: Vec<Card> = cards
        .iter()
        .filter(|card| card.1 == Suit::Club)
        .cloned()
        .collect::<Vec<Card>>();
    let diamonds: Vec<Card> = cards
        .iter()
        .filter(|card| card.1 == Suit::Diamond)
        .cloned()
        .collect::<Vec<Card>>();
    return [spades, hearts, clubs, diamonds];
}

fn get_flush(cards: &[Card]) -> Option<HandValue> {
    let suits = split_suits(cards);
    for suit in suits {
        if suit.len() >= 5 {
            return match get_straight(&suit) {
                Some(HandValue::Straight(rank)) => Some(HandValue::StraightFlush(rank)),
                Some(_) => None,
                None => Some(HandValue::Flush(
                    suit[suit.len() - 1].0,
                    suit[suit.len() - 2].0,
                    suit[suit.len() - 3].0,
                    suit[suit.len() - 4].0,
                    suit[suit.len() - 5].0,
                )),
            };
        }
    }
    return None;
}

pub fn get_best_hand(cards: &[Card]) -> Option<HandValue> {
    let multiples = get_best_multiple(cards);
    let straights = get_straight(cards);
    let flushed = get_flush(cards);
    let big = multiples.max(straights);
    return big.max(flushed);
}
