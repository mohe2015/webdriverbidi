# WebDriverBiDi Library

## Overview

The WebDriverBiDi library provides an interface for interacting with web browsers through the WebDriver BiDi (Bidirectional) protocol. This library allows you to create and manage WebDriver sessions, send commands, and handle responses asynchronously through WebSockets.

## Features

- Create and manage WebDriver sessions
- Connect to WebSocket URLs for bidirectional communication
- Send JSON commands to the WebSocket
- Handle incoming messages asynchronously
- Perform browsing context operations like getting the tree and navigating

## Getting Started

### Prerequisites

- Rust and Cargo installed
- A WebDriver server that supports the BiDi protocol

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
webdriverbidi = "0.1.2"
```

### Usage

```rust
use tokio;
use webdriverbidi::models::remote::browsing_context::{
    GetTreeParameters, NavigateParameters, ReadinessState,
};
use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::webdriver::capabilities::{Capabilities, CapabilityRequest};

#[tokio::main]
async fn main() {
    // Step 1: Define the capabilities for the WebDriver session
    let always_match = CapabilityRequest::new();
    let capabilities = Capabilities::new(always_match);

    // Step 2: Create a new WebDriver BiDi session
    let mut bidi_session = WebDriverBiDiSession::new("localhost".to_string(), 4444, capabilities)
        .await
        .expect("Failed to connect to WebSocket");

    // Step 3: Get the browsing context tree
    let get_tree_params = GetTreeParameters::new(None, None);
    let get_tree_rslt = bidi_session
        .browsing_context_get_tree(get_tree_params)
        .await
        .expect("Failed to send command");

    // Step 4: Navigate to the Rust programming language website
    let navigate_params = NavigateParameters::new(
        get_tree_rslt.contexts[0].context.clone(),
        "https://www.rust-lang.org/".to_string(),
        Some(ReadinessState::Complete),
    );
    let _ = bidi_session
        .browsing_context_navigate(navigate_params)
        .await
        .expect("Failed to send command");

    // Admire the page for a few seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Step 5: Close the session
    bidi_session.close().await.expect("Failed to close session");
}
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

This is a temporary readme and will be replaced by a more detailed version later.