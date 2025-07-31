use wasm_bindgen::prelude::*;

mod ecs;
mod game;

use ecs::World;
use game::{Deck, Card};

/// High level game wrapper exposed to JavaScript.
/// This struct owns the ECS `World` and a deck of cards.
#[wasm_bindgen]
pub struct SolitaireGame {
    world: World,
    deck: Deck,
}

#[wasm_bindgen]
impl SolitaireGame {
    /// Create a new solitaire game with an empty ECS world and a full deck.
    #[wasm_bindgen(constructor)]
    pub fn new() -> SolitaireGame {
        SolitaireGame {
            world: World::new(),
            deck: Deck::standard(),
        }
    }

    /// Draw a card from the deck. Returns `None` when the deck is empty.
    pub fn draw_card(&mut self) -> Option<String> {
        self.deck.cards.pop().map(|c| format!("{:?} of {:?}", c.rank, c.suit))
    }
}
