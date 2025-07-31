// Basic Solitaire game structures implemented in Rust.
// This file contains the core data types used to model the game state.
// Everything is documented thoroughly so beginners can easily follow along.

// We import a few utilities from the `rand` crate to shuffle the deck.
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::ecs::Entity;

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

    /// Shuffle the deck using a random number generator.
    ///
    /// We rely on the `rand` crate so that the shuffle works the same on
    /// native and WASM targets.
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

/// Represents the different piles a card can belong to in Solitaire.
///
/// We keep this structure very small so it is easy to store as a component in
/// the ECS `World`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pile {
    /// The facedown stock pile that players draw cards from.
    Stock,
    /// The faceup waste pile where drawn cards go.
    Waste,
    /// One of the four foundation piles where cards are stacked by suit.
    Foundation(u8),
    /// One of the seven tableau piles used during play.
    Tableau(u8),
}

/// Simple component used to mark whether a card is face up on the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FaceUp(pub bool);

