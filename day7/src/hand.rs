#[derive(Debug, Clone)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn get_numeric_value(&self) -> u64 {
        match self {
            Self::FiveOfAKind => 60000000000,
            Self::FourOfAKind => 50000000000,
            Self::FullHouse => 40000000000,
            Self::ThreeOfAKind => 30000000000,
            Self::TwoPair => 20000000000,
            Self::OnePair => 10000000000,
            Self::HighCard => 0,
        }
    }
}

const RANKED_CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const RANKED_CARDS_P2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: [char; 5],
    pub hand_type: HandType,
    pub bid: usize,
    pub numeric_value: u64,
}

impl Hand {
    pub fn from_str(input: &str, is_part_2: bool) -> Self {
        let (cards_str, bid_str) = input.split_once(' ').unwrap();

        let ranked_cards = if is_part_2 {
            RANKED_CARDS_P2
        } else {
            RANKED_CARDS
        };

        let cards = cards_str.chars();

        let mut numeric_value = 0;
        let mut cards_by_rank: [usize; 13] = Default::default();
        let mut wildcards = 0;
        for (i, card) in cards.clone().enumerate() {
            let factor = (10u64).pow((4 - i) as u32 * 2);
            let rank = ranked_cards.iter().position(|c| c.eq(&card)).unwrap();
            numeric_value += rank as u64 * factor;
            cards_by_rank[rank] += 1;
            if is_part_2 && card == 'J' {
                wildcards += 1;
            }
        }

        // Remove wildcards from calculation
        if is_part_2 {
            cards_by_rank[0] = 0;
        }

        cards_by_rank.sort();
        cards_by_rank.reverse();
        cards_by_rank[0] += wildcards;

        // Determine hand type
        let hand_type = match cards_by_rank[0..2] {
            [a, _] if a.eq(&5usize) => HandType::FiveOfAKind,
            [a, _] if a.eq(&4usize) => HandType::FourOfAKind,
            [a, b] if a.eq(&3usize) && b.eq(&2usize) => HandType::FullHouse,
            [a, b] if a.eq(&3usize) && b.eq(&1usize) => HandType::ThreeOfAKind,
            [a, b] if a.eq(&2usize) && b.eq(&2usize) => HandType::TwoPair,
            [a, b] if a.eq(&2usize) && b.eq(&1usize) => HandType::OnePair,
            _ => HandType::HighCard,
        };

        numeric_value += hand_type.get_numeric_value();

        Self {
            cards: cards.collect::<Vec<char>>().try_into().unwrap(),
            hand_type,
            bid: bid_str.parse().unwrap(),
            numeric_value,
        }
    }
}
