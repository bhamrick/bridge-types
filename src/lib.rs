use serde::{Deserialize, Serialize};

use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Ord for Suit {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Suit::Spades, Suit::Spades) => Ordering::Equal,
            (Suit::Spades, Suit::Hearts) => Ordering::Greater,
            (Suit::Spades, Suit::Diamonds) => Ordering::Greater,
            (Suit::Spades, Suit::Clubs) => Ordering::Greater,
            (Suit::Hearts, Suit::Spades) => Ordering::Less,
            (Suit::Hearts, Suit::Hearts) => Ordering::Equal,
            (Suit::Hearts, Suit::Diamonds) => Ordering::Greater,
            (Suit::Hearts, Suit::Clubs) => Ordering::Greater,
            (Suit::Diamonds, Suit::Spades) => Ordering::Less,
            (Suit::Diamonds, Suit::Hearts) => Ordering::Less,
            (Suit::Diamonds, Suit::Diamonds) => Ordering::Equal,
            (Suit::Diamonds, Suit::Clubs) => Ordering::Greater,
            (Suit::Clubs, Suit::Spades) => Ordering::Less,
            (Suit::Clubs, Suit::Hearts) => Ordering::Less,
            (Suit::Clubs, Suit::Diamonds) => Ordering::Less,
            (Suit::Clubs, Suit::Clubs) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Strain {
    NoTrump,
    Suit(Suit),
}

impl Ord for Strain {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Strain::NoTrump, Strain::NoTrump) => Ordering::Equal,
            (Strain::NoTrump, Strain::Suit(_)) => Ordering::Greater,
            (Strain::Suit(_), Strain::NoTrump) => Ordering::Less,
            (Strain::Suit(s1), Strain::Suit(s2)) => s1.cmp(s2),
        }
    }
}

impl PartialOrd for Strain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::convert::From<Suit> for Strain {
    fn from(suit: Suit) -> Strain {
        Strain::Suit(suit)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Seat {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SeatRelation {
    Me,
    LHO,
    Partner,
    RHO,
}

impl Seat {
    /// Next player in play order.
    ///
    /// Same as [Self::lho].
    pub fn next(self) -> Self {
        match self {
            Seat::North => Seat::East,
            Seat::East => Seat::South,
            Seat::South => Seat::West,
            Seat::West => Seat::North,
        }
    }

    /// Partner.
    pub fn partner(self) -> Self {
        match self {
            Seat::North => Seat::South,
            Seat::East => Seat::West,
            Seat::South => Seat::North,
            Seat::West => Seat::East,
        }
    }

    /// Left hand opponent.
    ///
    /// Same as [Self::next].
    pub fn lho(self) -> Self {
        match self {
            Seat::North => Seat::East,
            Seat::East => Seat::South,
            Seat::South => Seat::West,
            Seat::West => Seat::North,
        }
    }

    /// Right hand opponent.
    pub fn rho(self) -> Self {
        match self {
            Seat::North => Seat::West,
            Seat::East => Seat::North,
            Seat::South => Seat::East,
            Seat::West => Seat::South,
        }
    }

    pub fn side(self) -> Side {
        match self {
            Seat::North => Side::NS,
            Seat::East => Side::EW,
            Seat::South => Side::NS,
            Seat::West => Side::EW,
        }
    }

    pub fn relation_to(self, other: Seat) -> SeatRelation {
        if self == other {
            SeatRelation::Me
        } else if self == other.next() {
            SeatRelation::LHO
        } else if self == other.partner() {
            SeatRelation::Partner
        } else {
            SeatRelation::RHO
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Side {
    NS,
    EW,
}

impl Side {
    pub fn opponents(self) -> Side {
        match self {
            Side::NS => Side::EW,
            Side::EW => Side::NS,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Holding(pub u32);

impl Holding {
    pub fn new() -> Self {
        Holding(0)
    }

    pub fn add(&mut self, rank: u32) {
        self.0 |= 1 << rank;
    }

    pub fn remove(&mut self, rank: u32) {
        self.0 &= !(1 << rank);
    }

    pub fn contains(&self, rank: u32) -> bool {
        self.0 & (1 << rank) != 0
    }

    pub fn iter(self) -> HoldingIterator {
        let mut front = 2;
        while front < 15 && !self.contains(front) {
            front += 1;
        }
        let mut back = 14;
        while back > 1 && !self.contains(back) {
            back -= 1;
        }
        HoldingIterator {
            holding: self,
            front,
            back,
        }
    }

    pub fn count(self) -> u32 {
        self.0.count_ones()
    }
}

impl std::iter::IntoIterator for Holding {
    type Item = u32;
    type IntoIter = HoldingIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl std::iter::FromIterator<u32> for Holding {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        let mut holding = Holding::new();

        for i in iter {
            holding.add(i);
        }

        holding
    }
}

#[derive(Copy, Clone, Debug)]
pub struct HoldingIterator {
    holding: Holding,
    front: u32,
    back: u32,
}

impl std::iter::Iterator for HoldingIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front > self.back {
            None
        } else {
            let ret = Some(self.front);
            self.front += 1;
            while self.front < 15 && !self.holding.contains(self.front) {
                self.front += 1;
            }
            ret
        }
    }
}

impl std::iter::DoubleEndedIterator for HoldingIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front > self.back {
            None
        } else {
            let ret = Some(self.back);
            self.back -= 1;
            while self.back > 1 && !self.holding.contains(self.back) {
                self.back -= 1;
            }
            ret
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerSuit<T> {
    pub spades: T,
    pub hearts: T,
    pub diamonds: T,
    pub clubs: T,
}

impl<T: Copy> PerSuit<T> {
    pub fn new(value: T) -> Self {
        PerSuit {
            spades: value,
            hearts: value,
            diamonds: value,
            clubs: value,
        }
    }
}

impl<T> PerSuit<T> {
    pub fn new_with<F: FnMut() -> T>(mut get_value: F) -> Self {
        PerSuit {
            spades: get_value(),
            hearts: get_value(),
            diamonds: get_value(),
            clubs: get_value(),
        }
    }

    pub fn map<S, F: Fn(T) -> S>(self, f: F) -> PerSuit<S> {
        PerSuit {
            spades: f(self.spades),
            hearts: f(self.hearts),
            diamonds: f(self.diamonds),
            clubs: f(self.clubs),
        }
    }

    pub fn map_with_suit<S, F: Fn(Suit, T) -> S>(self, f: F) -> PerSuit<S> {
        PerSuit {
            spades: f(Suit::Spades, self.spades),
            hearts: f(Suit::Hearts, self.hearts),
            diamonds: f(Suit::Diamonds, self.diamonds),
            clubs: f(Suit::Clubs, self.clubs),
        }
    }

    pub fn iter<'a>(&'a self) -> PerSuitIter<'a, T> {
        PerSuitIter {
            source: self,
            index: Some(Suit::Spades),
        }
    }
}

impl<T: std::ops::Add<T, Output = T>> PerSuit<T> {
    pub fn sum(self) -> T {
        self.spades + self.hearts + self.diamonds + self.clubs
    }
}

impl<T> std::ops::Index<Suit> for PerSuit<T> {
    type Output = T;

    fn index(&self, index: Suit) -> &Self::Output {
        match index {
            Suit::Spades => &self.spades,
            Suit::Hearts => &self.hearts,
            Suit::Diamonds => &self.diamonds,
            Suit::Clubs => &self.clubs,
        }
    }
}

impl<T> std::ops::IndexMut<Suit> for PerSuit<T> {
    fn index_mut(&mut self, index: Suit) -> &mut Self::Output {
        match index {
            Suit::Spades => &mut self.spades,
            Suit::Hearts => &mut self.hearts,
            Suit::Diamonds => &mut self.diamonds,
            Suit::Clubs => &mut self.clubs,
        }
    }
}

pub struct PerSuitIter<'a, T> {
    source: &'a PerSuit<T>,
    index: Option<Suit>,
}

impl<'a, T> Iterator for PerSuitIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.index.map(|idx| &self.source[idx]);
        self.index = match self.index {
            Some(Suit::Spades) => Some(Suit::Hearts),
            Some(Suit::Hearts) => Some(Suit::Diamonds),
            Some(Suit::Diamonds) => Some(Suit::Clubs),
            Some(Suit::Clubs) => None,
            None => None,
        };
        ret
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerSeat<T> {
    pub north: T,
    pub east: T,
    pub south: T,
    pub west: T,
}

impl<T: Copy> PerSeat<T> {
    pub fn new(value: T) -> Self {
        PerSeat {
            north: value,
            east: value,
            south: value,
            west: value,
        }
    }
}

impl<T> PerSeat<T> {
    pub fn new_with<F: FnMut() -> T>(mut get_value: F) -> Self {
        PerSeat {
            north: get_value(),
            east: get_value(),
            south: get_value(),
            west: get_value(),
        }
    }

    pub fn map<S, F: Fn(T) -> S>(self, f: F) -> PerSeat<S> {
        PerSeat {
            north: f(self.north),
            east: f(self.east),
            south: f(self.south),
            west: f(self.west),
        }
    }

    pub fn map_with_seat<S, F: Fn(Seat, T) -> S>(self, f: F) -> PerSeat<S> {
        PerSeat {
            north: f(Seat::North, self.north),
            east: f(Seat::East, self.east),
            south: f(Seat::South, self.south),
            west: f(Seat::West, self.west),
        }
    }

    pub fn iter<'a>(&'a self) -> PerSeatIter<'a, T> {
        PerSeatIter {
            source: self,
            index: Some(Seat::North),
        }
    }
}

impl<T> std::ops::Index<Seat> for PerSeat<T> {
    type Output = T;

    fn index(&self, index: Seat) -> &Self::Output {
        match index {
            Seat::North => &self.north,
            Seat::East => &self.east,
            Seat::South => &self.south,
            Seat::West => &self.west,
        }
    }
}

impl<T> std::ops::IndexMut<Seat> for PerSeat<T> {
    fn index_mut(&mut self, index: Seat) -> &mut Self::Output {
        match index {
            Seat::North => &mut self.north,
            Seat::East => &mut self.east,
            Seat::South => &mut self.south,
            Seat::West => &mut self.west,
        }
    }
}

pub struct PerSeatIter<'a, T> {
    source: &'a PerSeat<T>,
    index: Option<Seat>,
}

impl<'a, T> Iterator for PerSeatIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.index.map(|idx| &self.source[idx]);
        self.index = match self.index {
            Some(Seat::North) => Some(Seat::East),
            Some(Seat::East) => Some(Seat::South),
            Some(Seat::South) => Some(Seat::West),
            Some(Seat::West) => None,
            None => None,
        };
        ret
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerStrain<T> {
    pub notrump: T,
    pub spades: T,
    pub hearts: T,
    pub diamonds: T,
    pub clubs: T,
}

impl<T: Copy> PerStrain<T> {
    pub fn new(value: T) -> Self {
        PerStrain {
            notrump: value,
            spades: value,
            hearts: value,
            diamonds: value,
            clubs: value,
        }
    }
}

impl<T> std::ops::Index<Strain> for PerStrain<T> {
    type Output = T;

    fn index(&self, index: Strain) -> &Self::Output {
        match index {
            Strain::NoTrump => &self.notrump,
            Strain::Suit(Suit::Spades) => &self.spades,
            Strain::Suit(Suit::Hearts) => &self.hearts,
            Strain::Suit(Suit::Diamonds) => &self.diamonds,
            Strain::Suit(Suit::Clubs) => &self.clubs,
        }
    }
}

impl<T> std::ops::IndexMut<Strain> for PerStrain<T> {
    fn index_mut(&mut self, index: Strain) -> &mut Self::Output {
        match index {
            Strain::NoTrump => &mut self.notrump,
            Strain::Suit(Suit::Spades) => &mut self.spades,
            Strain::Suit(Suit::Hearts) => &mut self.hearts,
            Strain::Suit(Suit::Diamonds) => &mut self.diamonds,
            Strain::Suit(Suit::Clubs) => &mut self.clubs,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerSide<T> {
    pub ns: T,
    pub ew: T,
}

impl<T: Copy> PerSide<T> {
    pub fn new(value: T) -> Self {
        PerSide {
            ns: value,
            ew: value,
        }
    }
}

impl<T> PerSide<T> {
    pub fn map<S, F: Fn(T) -> S>(self, f: F) -> PerSide<S> {
        PerSide {
            ns: f(self.ns),
            ew: f(self.ew),
        }
    }
}

impl<T> std::ops::Index<Side> for PerSide<T> {
    type Output = T;

    fn index(&self, index: Side) -> &Self::Output {
        match index {
            Side::NS => &self.ns,
            Side::EW => &self.ew,
        }
    }
}

impl<T> std::ops::IndexMut<Side> for PerSide<T> {
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        match index {
            Side::NS => &mut self.ns,
            Side::EW => &mut self.ew,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Contract {
    pub level: u8,
    pub strain: Strain,
    pub doubling: Doubling,
}

impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.level,
            match self.strain {
                Strain::NoTrump => "NT",
                Strain::Suit(Suit::Spades) => "S",
                Strain::Suit(Suit::Hearts) => "H",
                Strain::Suit(Suit::Diamonds) => "D",
                Strain::Suit(Suit::Clubs) => "C",
            },
            match self.doubling {
                Doubling::Undoubled => "",
                Doubling::Doubled => "x",
                Doubling::Redoubled => "xx",
            }
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Doubling {
    Undoubled,
    Doubled,
    Redoubled,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Call {
    Pass,
    Bid(u8, Strain),
    Double,
    Redouble,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Card(pub u32, pub Suit);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holding_iterator() {
        let holding = Holding(0x124);
        let held_cards: Vec<u32> = holding.iter().collect();
        assert_eq!(held_cards, vec![2, 5, 8]);
    }
}
