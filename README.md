# webdriverbidi

## Overview

The webdriverbidi library provides an interface for interacting with web browsers through the WebDriver BiDi (Bidirectional) protocol. This library allows you to create and manage WebDriver sessions, send commands, and handle responses asynchronously through WebSockets.

## Features

- Create and manage WebDriver BiDi sessions
- Send commands
- Handle events asynchronously

## Getting Started

### Prerequisites

- Rust and Cargo installed
- A WebDriver server that supports the BiDi protocol

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
webdriverbidi = "0.1.6"
```

### Usage

Start a WebDriver BiDi compliant server

```bash
$ geckodriver --host=localhost --port=4444
```

Create a new Rust project and add the following code to `src/main.rs`:

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

## Module Coverate

### session
#### Types
#### Commands

### browser
#### Types
#### Commands

### browsingContext
#### Types
- [x] browsingContext.BrowsingContext
- [x] browsingContext.Info
- [x] browsingContext.Locator
- [x] browsingContext.Navigation
- [x] browsingContext.NavigationInfo
- [x] browsingContext.ReadinessState
- [x] browsingContext.UserPromptType

#### Commands
- [ ] browsingContext.activate
- [ ] browsingContext.captureScreenshot
- [ ] browsingContext.close
- [ ] browsingContext.create
- [x] browsingContext.getTree
- [ ] browsingContext.handleUserPrompt
- [ ] browsingContext.locateNodes
- [x] browsingContext.navigate
- [ ] browsingContext.print
- [ ] browsingContext.reload
- [ ] browsingContext.setViewport
- [ ] browsingContext.traverseHistory

#### Events
- [ ] browsingContext.contextCreated
- [ ] browsingContext.contextDestroyed
- [ ] browsingContext.navigationStarted
- [ ] browsingContext.fragmentNavigated
- [ ] browsingContext.historyUpdated
- [ ] browsingContext.domContentLoaded
- [ ] browsingContext.load
- [ ] browsingContext.downloadWillBegin
- [ ] browsingContext.navigationAborted
- [ ] browsingContext.navigationFailed
- [ ] browsingContext.userPromptClosed
- [ ] browsingContext.userPromptOpened

### network
#### Types
#### Commands
#### Events

### script
#### Types
#### Commands
#### Events

### storage
#### Types
#### Commands


### log
#### Types
#### Commands

### input
#### Types
#### Commands

### webExtension
#### Types
#### Commands

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

This is a temporary readme and will be replaced by a more detailed version later.