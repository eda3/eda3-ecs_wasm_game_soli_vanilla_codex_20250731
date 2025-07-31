//! A small networking utility to communicate with a server via WebSockets.
//!
//! This module is written for educational purposes so every step is heavily
//! commented. It exposes a `NetworkClient` that can be created from JavaScript
//! using `wasm-bindgen`. The client wraps the browser's `WebSocket` API and
//! allows sending text messages and registering callbacks for incoming
//! messages or connection events.

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{BinaryType, ErrorEvent, Event, MessageEvent, WebSocket};

/// A very small wrapper around `WebSocket` so that we can use it from Rust
/// and expose it to JavaScript through WebAssembly.
#[wasm_bindgen]
pub struct NetworkClient {
    /// The underlying WebSocket handle provided by the browser.
    ws: WebSocket,
}

#[wasm_bindgen]
impl NetworkClient {
    /// Create and connect to a WebSocket at the given URL.
    ///
    /// The constructor returns a `Result` because establishing the connection
    /// might fail if the URL is invalid or the browser blocks it.
    #[wasm_bindgen(constructor)]
    pub fn new(url: &str) -> Result<NetworkClient, JsValue> {
        let ws = WebSocket::new(url)?;
        ws.set_binary_type(BinaryType::Arraybuffer);
        Ok(NetworkClient { ws })
    }

    /// Send a UTF-8 text message to the server.
    pub fn send(&self, msg: &str) -> Result<(), JsValue> {
        self.ws.send_with_str(msg)
    }

    /// Set a callback that is invoked whenever a message is received.
    ///
    /// The callback receives the text of the message as its only argument.
    pub fn on_message(&self, callback: &js_sys::Function) {
        // Clone the function so it can be moved into the `Closure` and live
        // for the entire lifetime of the websocket.
        let cb_func = callback.clone();
        let cb = Closure::<dyn FnMut(MessageEvent)>::wrap(Box::new(move |e| {
            if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                let _ = cb_func.call1(&JsValue::NULL, &text);
            }
        }));
        self.ws.set_onmessage(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    /// Set a callback that fires when the socket is successfully opened.
    pub fn on_open(&self, callback: &js_sys::Function) {
        let cb_func = callback.clone();
        let cb = Closure::<dyn FnMut(Event)>::wrap(Box::new(move |_| {
            let _ = cb_func.call0(&JsValue::NULL);
        }));
        self.ws.set_onopen(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    /// Set a callback that fires if an error occurs with the connection.
    pub fn on_error(&self, callback: &js_sys::Function) {
        let cb_func = callback.clone();
        let cb = Closure::<dyn FnMut(ErrorEvent)>::wrap(Box::new(move |e| {
            let _ = cb_func.call1(&JsValue::NULL, &JsValue::from(e.message()));
        }));
        self.ws.set_onerror(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }
}
