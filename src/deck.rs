use self::Rank::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl Rank {
    pub fn iterator() -> impl Iterator<Item = Rank> {
        [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, J, Q, K, A,
        ]
        .iter()
        .copied()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Card(pub Rank, pub Suit);

#[derive(Debug)]
pub struct Deck {
    rng: ThreadRng,
    pub cards: [Card; 52],
}

impl Deck {
    pub fn new() -> Deck {
        return Deck {
            cards: core::array::from_fn(|i| Card(rank_from_index(i % 13), suit_from_index(i / 13))),
            rng: thread_rng(),
        };
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut self.rng);
    }
}

type RankCount = [usize; 13];

pub fn rank_from_index(rank_index: usize) -> Rank {
    return match rank_index {
        0 => Rank::Two,
        1 => Rank::Three,
        2 => Rank::Four,
        3 => Rank::Five,
        4 => Rank::Six,
        5 => Rank::Seven,
        6 => Rank::Eight,
        7 => Rank::Nine,
        8 => Rank::Ten,
        9 => Rank::J,
        10 => Rank::Q,
        11 => Rank::K,
        12 => Rank::A,
        _ => Rank::A,
    };
}

pub fn index_from_rank(rank: &Rank) -> usize {
    return match rank {
        Rank::Two => 0,
        Rank::Three => 1,
        Rank::Four => 2,
        Rank::Five => 3,
        Rank::Six => 4,
        Rank::Seven => 5,
        Rank::Eight => 6,
        Rank::Nine => 7,
        Rank::Ten => 8,
        Rank::J => 9,
        Rank::Q => 10,
        Rank::K => 11,
        Rank::A => 12,
    };
}

pub fn suit_from_index(suit_index: usize) -> Suit {
    return match suit_index {
        0 => Suit::Spade,
        1 => Suit::Heart,
        2 => Suit::Club,
        3 => Suit::Diamond,
        _ => Suit::Spade,
    };
}

fn increment_rank(card: &Card, counter: &RankCount) -> RankCount {
    let mut mycount = counter.clone();
    mycount[index_from_rank(&card.0)] += 1;
    return mycount;
}

pub fn count_ranks(cards: &[Card]) -> RankCount {
    return cards
        .iter()
        .fold([0; 13], |acc, card| increment_rank(card, &acc));
}

pub fn get_count_ranks(counts: RankCount) -> [Vec<Rank>; 5] {
    let mut count_ranks: [Vec<Rank>; 5] = [vec![], vec![], vec![], vec![], vec![]];
    for (rank_index, count) in counts.iter().enumerate() {
        count_ranks[*count].push(rank_from_index(rank_index));
    }
    return count_ranks;
}
