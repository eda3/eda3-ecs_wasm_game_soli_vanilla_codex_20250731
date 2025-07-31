use wasm_bindgen::prelude::*;

mod ecs;
mod game;
mod network;

use ecs::{Entity, World};
use game::{Card, Deck, Pile, FaceUp};
use network::NetworkClient;

/// High level game wrapper exposed to JavaScript.
/// This struct owns the ECS `World` and a deck of cards.
#[wasm_bindgen]
pub struct SolitaireGame {
    world: World,
    deck: Deck,
    // Networking is optional. We create the socket lazily when the player
    // decides to join a multiplayer session.
    network: Option<NetworkClient>,
}

#[wasm_bindgen]
impl SolitaireGame {
    /// Create a new solitaire game with an empty ECS world and a full deck.
    #[wasm_bindgen(constructor)]
    pub fn new() -> SolitaireGame {
        SolitaireGame {
            world: World::new(),
            deck: Deck::standard(),
            network: None,
        }
    }

    /// Draw a card from the deck. Returns `None` when the deck is empty.
    pub fn draw_card(&mut self) -> Option<String> {
        self.deck
            .cards
            .pop()
            .map(|c| format!("{:?} of {:?}", c.rank, c.suit))
    }

    /// Set up a fresh solitaire board by shuffling the deck and dealing the
    /// cards into their initial piles.
    ///
    /// This method demonstrates how to spawn entities and attach components in
    /// our tiny ECS. It does not implement every solitaire rule, but it
    /// prepares the tableau, foundations, stock and waste piles so that the
    /// game logic can be built on top.
    pub fn setup_board(&mut self) {
        // Reset the ECS world and shuffle the deck so every game is different.
        self.world = World::new();
        self.deck.shuffle();

        // We will spawn an entity for each card in the deck and attach the
        // relevant components.
        for card in self.deck.cards.iter() {
            // Create a new entity identifier.
            let entity = self.world.spawn();

            // Every entity gets a `Card` component storing its suit and rank.
            self.world.add_component(entity, *card);

            // Cards start face down by default.
            self.world.add_component(entity, FaceUp(false));

            // Place the card into the stock pile. A real game would deal cards
            // to the tableau here, but keeping it simple lets beginners focus
            // on the ECS mechanics first.
            self.world.add_component(entity, Pile::Stock);
        }
    }

    /// Connect to a multiplayer server using a WebSocket URL.
    ///
    /// Returns an error if the connection could not be established.
    pub fn connect(&mut self, url: &str) -> Result<(), JsValue> {
        let client = NetworkClient::new(url)?;
        self.network = Some(client);
        Ok(())
    }

    /// Send a text message over the WebSocket if it is connected.
    pub fn send(&self, msg: &str) -> Result<(), JsValue> {
        match &self.network {
            Some(net) => net.send(msg),
            None => Err(JsValue::from_str("Not connected")),
        }
    }
}
