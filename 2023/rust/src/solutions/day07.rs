use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
    Joker,
}

impl TryFrom<char> for Card {
    type Error = String;
    fn try_from(c: char) -> Result<Card, Self::Error>  {
        match c{
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::N9),
            '8' => Ok(Card::N8),
            '7' => Ok(Card::N7),
            '6' => Ok(Card::N6),
            '5' => Ok(Card::N5),
            '4' => Ok(Card::N4),
            '3' => Ok(Card::N3),
            '2' => Ok(Card::N2),
            c => Err(format!("unknown card {c}"))
        }
    }
}


#[derive(Debug,Clone,Copy, PartialEq,Eq,PartialOrd,Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl<T> From<T> for HandType
where T: AsRef<[Card]> {
    fn from(cards: T) -> Self {
        //TODO
        let mut ordered: Vec<Card> = cards.as_ref().iter().copied().collect();
        ordered.sort();
        // dbg!(&ordered);
        let mut last = ordered.pop().unwrap();
        let mut count = 1;
        let mut counts: [usize; 5] = [0; 5];
        
        while let Some(card) = ordered.pop() {
            if card == last {
                count += 1;
            } else {
                counts[count-1] += 1;
                last = card;
                count = 1;
            }
        }
        if count > 0 {
            counts[count-1] += 1;
        }
        match counts {
            [0, 0, 0, 0, 1] => Self::FiveOfAKind,
            [1, 0, 0, 1, 0] => Self::FourOfAKind,
            [0, 1, 1, 0, 0] => Self::FullHouse,
            [2, 0, 1, 0, 0] => Self::ThreeOfAKind,
            [1, 2, 0, 0, 0] => Self::TwoPair,
            [3, 1, 0, 0, 0] => Self::OnePair,
            [5, 0, 0, 0, 0] => Self::HighCard,
            _ => panic!("unknown hand {counts:?} {:?}", cards.as_ref())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: usize,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl Eq for Hand {}

impl Hand {
    fn new(input: &str, bid: usize) -> Self {
        let cards: Vec<Card> = input.chars().map(|c| Card::try_from(c).expect("invalid card {c}")).collect();
        let hand_type = (&cards).into();
        Hand{cards, hand_type, bid}
    }
}



impl PartialOrd for Hand{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type).then_with(||
            other.cards.cmp(&self.cards)
        )
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
struct JokerHand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: usize,
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type).then_with(||
            other.cards.cmp(&self.cards)
        )
    }
}

impl From<&Hand> for JokerHand {
    fn from(hand: &Hand) -> Self {
        let cards: Vec<Card> = hand.cards.iter().map(|c| {
            match c {
                Card::J => Card::Joker,
                c => c.clone(),
            }
            }).collect();
        let mut options = vec![cards.clone()];
        use Card::*;
        let values: [Card; 12] = [ A, K, Q, T, N9, N8, N7, N6, N5, N4, N3, N2];
        for ix in 0..5 {
            let mut jx = 0;
            while jx < options.len() {
                if options[jx][ix] == Card::Joker {
                    let mut hand = options.swap_remove(jx);
                    for val in values.iter().copied() {
                        hand[ix] = val;
                        let h = hand.clone();
                        options.push(h);
                    }
                } else {
                    jx += 1;
                }
            }
        }
        // dbg!(&options);
        let mut options: Vec<HandType> = options.into_iter().map(|c| c.into()).collect();
        options.sort();
        let hand_type = options.last().expect("no options").clone();
        // println!("Best hand type for {:?} is {:?}", hand, hand_type);
        JokerHand{cards, hand_type, bid: hand.bid}
    }
}

pub fn parse(input: &str) -> Vec<Hand> {
    let mut hands: Vec<_> = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (cards, bid) = line.split_once(' ').expect("could not split in two");
            let bid = bid.parse::<usize>().expect("invalid bid {bid}");
            Hand::new(cards, bid)
        }).collect();
    hands.sort();
    hands
}

pub fn part1(hands: &[Hand]) -> usize {
    // dbg!(&hands);
    hands.iter().enumerate().map(|(ix, hand)| {
        (ix+1) * hand.bid
    }).sum()
}

pub fn part2(hands: &[Hand]) -> usize {
    let mut hands: Vec<JokerHand> = hands.iter().map(|h| h.into()).collect();
    hands.sort();
    hands.iter().enumerate().map(|(ix, hand)| {
        (ix+1) * hand.bid
    }).sum()
}
