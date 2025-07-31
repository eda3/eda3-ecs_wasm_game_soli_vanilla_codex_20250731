use wasm_bindgen::prelude::*;

mod ecs;
mod game;
mod network;

use ecs::World;
use game::{Card, Deck};
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
