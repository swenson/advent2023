use std::cmp::Ordering;

pub fn day7() {
    let _inp = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let inp = std::fs::read_to_string("day7.input.txt").unwrap();
    let inp = inp.trim();

    let mut hands = vec![];
    for line in inp.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let bid: u64 = parts[1].parse().unwrap();
        hands.push(Hand::new(parts[0].trim(), bid));
    }
    hands.sort();
    let mut winnings = 0;
    for (rank, hand) in hands.iter().enumerate() {
        winnings += (rank + 1) as u64 * hand.bid;
    }
    println!("Winnings = {}", winnings);
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.counts == other.counts
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let max_count_self = self.counts.iter().max().unwrap();
        let max_count_other = other.counts.iter().max().unwrap();

        if max_count_self > max_count_other {
            return Ordering::Greater;
        }
        if max_count_self < max_count_other {
            return Ordering::Less;
        }

        let full_house_self = self.counts.contains(&3) && self.counts.contains(&2);
        let full_house_other = other.counts.contains(&3) && other.counts.contains(&2);
        if full_house_self && !full_house_other {
            return Ordering::Greater;
        }
        if !full_house_self && full_house_other {
            return Ordering::Less;
        }

        let two_pair_self = self.counts.iter().filter(|x| **x == 2).count() == 2;
        let two_pair_other = other.counts.iter().filter(|x| **x == 2).count() == 2;
        if two_pair_self && !two_pair_other {
            return Ordering::Greater;
        }
        if !two_pair_self && two_pair_other {
            return Ordering::Less;
        }

        for i in 0..5 {
            let a = rank_card(self.hand[i]);
            let b = rank_card(other.hand[i]);
            let cmp = a.cmp(&b);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }
        Ordering::Equal
    }

    fn max(self, _other: Self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn min(self, _other: Self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn clamp(self, _min: Self, _max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        todo!()
    }
}

#[derive(Debug)]
struct Hand {
    hand: Vec<char>,
    counts: [u8; 13],
    bid: u64,
}

fn rank_card(x: char) -> usize {
    match x {
        '2'..='9' => x as usize - '2' as usize,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Invalid character: {}", x),
    }
}

fn rank_card2(x: char) -> usize {
    match x {
        'J' => 0,
        '2'..='9' => x as usize - '2' as usize + 1,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Invalid character: {}", x),
    }
}

impl Hand {
    fn new(hand: &str, bid: u64) -> Hand {
        let mut counts = [0u8; 13];
        for x in hand.chars() {
            let idx = rank_card(x);
            counts[idx] += 1;
        }
        Hand {
            counts,
            bid,
            hand: hand.chars().collect(),
        }
    }
}

pub fn day7_2() {
    let _inp = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let inp = std::fs::read_to_string("day7.input.txt").unwrap();
    let inp = inp.trim();

    let mut hands = vec![];
    for line in inp.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let bid: u64 = parts[1].parse().unwrap();
        hands.push(Hand2::new(parts[0].trim(), bid));
    }
    hands.sort();
    let mut winnings = 0;
    for (rank, hand) in hands.iter().enumerate() {
        winnings += (rank + 1) as u64 * hand.bid;
    }
    println!("Winnings = {}", winnings);
}

impl Eq for Hand2 {}

impl PartialEq<Self> for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.counts == other.counts
    }
}

impl PartialOrd<Self> for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let max_count_self = self.counts.iter().max().unwrap();
        let max_count_other = other.counts.iter().max().unwrap();

        if max_count_self > max_count_other {
            return Ordering::Greater;
        }
        if max_count_self < max_count_other {
            return Ordering::Less;
        }

        let full_house_self = self.counts.contains(&3) && self.counts.contains(&2);
        let full_house_other = other.counts.contains(&3) && other.counts.contains(&2);
        if full_house_self && !full_house_other {
            return Ordering::Greater;
        }
        if !full_house_self && full_house_other {
            return Ordering::Less;
        }

        let two_pair_self = self.counts.iter().filter(|x| **x == 2).count() == 2;
        let two_pair_other = other.counts.iter().filter(|x| **x == 2).count() == 2;
        if two_pair_self && !two_pair_other {
            return Ordering::Greater;
        }
        if !two_pair_self && two_pair_other {
            return Ordering::Less;
        }

        for i in 0..5 {
            let a = rank_card2(self.hand[i]);
            let b = rank_card2(other.hand[i]);
            let cmp = a.cmp(&b);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }
        Ordering::Equal
    }

    fn max(self, _other: Self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn min(self, _other: Self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn clamp(self, _min: Self, _max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        todo!()
    }
}

#[derive(Debug)]
struct Hand2 {
    hand: Vec<char>,
    counts: [u8; 13],
    bid: u64,
}

impl Hand2 {
    fn new(hand: &str, bid: u64) -> Hand2 {
        let mut counts = [0u8; 13];
        for x in hand.chars() {
            let idx = rank_card2(x);
            counts[idx] += 1;
        }
        if counts[0] > 0 {
            let m = *counts.iter().skip(1).max().unwrap();
            for i in 1..13 {
                if counts[i] == m {
                    counts[i] += counts[0];
                    break;
                }
            }
            counts[0] = 0;
        }
        Hand2 {
            counts,
            bid,
            hand: hand.chars().collect(),
        }
    }
}
