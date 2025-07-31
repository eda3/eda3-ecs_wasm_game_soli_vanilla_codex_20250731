// Basic Solitaire game structures implemented in Rust.
// This file contains the core data types used to model the game state.
// Everything is documented thoroughly so beginners can easily follow along.

/// Represents the four suits found in a standard deck of cards.
/// Using an enum ensures each suit is a distinct value at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// Values for playing cards, ranging from Ace to King.
/// In solitaire we only need the rank information, so we use an enum here too.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

/// A simple card made of a `Suit` and `Rank`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    /// Create a new card with the given suit and rank.
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

/// A `Deck` is just a vector of cards.
/// We provide a constructor that builds a standard 52 card deck.
#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// Generate a full deck of 52 unique cards in order.
    pub fn standard() -> Self {
        let mut cards = Vec::with_capacity(52);
        for &suit in &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for &rank in &[
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ] {
                cards.push(Card::new(suit, rank));
            }
        }
        Self { cards }
    }
}

