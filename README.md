# webdriverbidi

## Overview

The webdriverbidi library provides an interface for interacting with web browsers through the WebDriver BiDi (Bidirectional) protocol. This library allows you to create and manage WebDriver sessions, send commands, and handle responses asynchronously through WebSockets.

## Planned Features

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
webdriverbidi = "0.1.10"
```

### Usage

Start a WebDriver BiDi compliant server

```bash
$ geckodriver --host=localhost --port=4444
```

Create a new Rust project and add the following code to `src/main.rs`:

```rust
use tokio;
use tokio::time;

// --------------------------------------------------

use webdriverbidi::remote::browsing_context::{
    GetTreeParameters, NavigateParameters, ReadinessState,
};
use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::webdriver::capabilities::{Capabilities, CapabilityRequest};

// --------------------------------------------------

async fn sleep(secs: u64) {
    time::sleep(time::Duration::from_secs(secs)).await
}

// --------------------------------------------------

#[tokio::main]
async fn main() {
    // Define the capabilities for the WebDriver session
    let always_match = CapabilityRequest::new();
    let capabilities = Capabilities::new(always_match);

    // Initialize a new WebDriver BiDi session and start it
    let host = String::from("localhost");
    let port = 4444;
    let mut bidi_session = WebDriverBiDiSession::new(host, port, capabilities);
    let _ = bidi_session.start().await.expect("Failed to start session");

    // Get the browsing context tree
    let get_tree_params = GetTreeParameters::new(None, None);
    let get_tree_rslt = bidi_session
        .browsing_context_get_tree(get_tree_params)
        .await
        .expect("Failed to send command");

    // Navigate to rust-lang.org
    let navigate_params = NavigateParameters::new(
        get_tree_rslt.contexts[0].context.clone(),
        "https://www.rust-lang.org/".to_string(),
        Some(ReadinessState::Complete),
    );
    let _ = bidi_session
        .browsing_context_navigate(navigate_params)
        .await
        .expect("Failed to send command");

    sleep(2).await;

    // Close the session
    bidi_session.close().await.expect("Failed to close session");
}
```

## Module Coverage

### session
#### Types
- [x] session.CapabilitiesRequest
- [x] session.CapabilityRequest
- [x] session.ProxyConfiguration
- [x] session.UserPromptHandler
- [x] session.UserPromptHandlerType
- [x] session.Subscription
- [x] session.SubscriptionRequest
- [x] session.UnsubscribeByIDRequest
- [x] session.UnsubscribeByAttributesRequest

#### Commands
- [x] session.status
- [x] session.new
- [x] session.end
- [x] session.subscribe
- [x] session.unsubscribe

### browser
#### Types
- [x] browser.ClientWindow
- [x] browser.ClientWindowInfo
- [x] browser.UserContext
- [x] browser.UserContextInfo

#### Commands
- [x] browser.close
- [x] browser.createUserContext
- [x] browser.getClientWindows
- [x] browser.getUserContexts
- [x] browser.removeUserContext
- [x] browser.setClientWindowState

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
- [x] browsingContext.activate
- [x] browsingContext.captureScreenshot
- [x] browsingContext.close
- [x] browsingContext.create
- [x] browsingContext.getTree
- [x] browsingContext.handleUserPrompt
- [x] browsingContext.locateNodes
- [x] browsingContext.navigate
- [x] browsingContext.print
- [x] browsingContext.reload
- [x] browsingContext.setViewport
- [x] browsingContext.traverseHistory

#### Events
- [x] browsingContext.contextCreated
- [x] browsingContext.contextDestroyed
- [x] browsingContext.navigationStarted
- [x] browsingContext.fragmentNavigated
- [x] browsingContext.historyUpdated
- [x] browsingContext.domContentLoaded
- [x] browsingContext.load
- [x] browsingContext.downloadWillBegin
- [x] browsingContext.navigationAborted
- [x] browsingContext.navigationFailed
- [x] browsingContext.userPromptClosed
- [x] browsingContext.userPromptOpened

### network
#### Types
- [x] network.AuthChallenge
- [x] network.AuthCredentials
- [x] network.BaseParameters
- [x] network.BytesValue
- [x] network.Cookie
- [x] network.CookieHeader
- [x] network.FetchTimingInfo
- [x] network.Header
- [x] network.Initiator
- [x] network.Intercept
- [x] network.Request
- [x] network.RequestData
- [x] network.ResponseContent
- [x] network.ResponseData
- [x] network.SetCookieHeader
- [x] network.UrlPattern

#### Commands
- [x] network.addIntercept
- [x] network.continueRequest
- [x] network.continueResponse
- [x] network.continueWithAuth
- [x] network.failRequest
- [x] network.provideResponse
- [x] network.removeIntercept
- [x] network.setCacheBehavior

#### Events
- [x] network.authRequired
- [x] network.beforeRequestSent
- [x] network.fetchError
- [x] network.responseCompleted
- [x] network.responseStarted

### script
#### Types
- [x] script.Channel
- [x] script.ChannelValue
- [x] script.EvaluateResult
- [x] script.ExceptionDetails
- [x] script.Handle
- [x] script.InternalId
- [x] script.LocalValue
- [x] script.PreloadScript
- [x] script.Realm
- [x] script.PrimitiveProtocolValue
- [x] script.RealmInfo
- [x] script.RealmType
- [x] script.RemoteReference
- [x] script.RemoteValue
- [x] script.ResultOwnership
- [x] script.SerializationOptions
- [x] script.SharedId
- [x] script.StackFrame
- [x] script.StackTrace
- [x] script.Source
- [x] script.Target

#### Commands
- [x] script.addPreloadScript
- [x] script.disown
- [x] script.callFunction
- [x] script.evaluate
- [x] script.getRealms
- [x] script.removePreloadScript

#### Events
- [x] script.message
- [x] script.realmCreated
- [x] script.realmDestroyed

### storage
#### Types
- [x] storage.PartitionKey

#### Commands
- [x] storage.getCookies
- [x] storage.setCookie
- [x] storage.deleteCookies

### log
#### Types
- [x] log.LogEntry

#### Events
- [x] log.entryAdded

### input
#### Types
- [x] input.ElementOrigin

#### Commands
- [x] input.performActions
- [x] input.releaseActions
- [x] input.setFiles

### webExtension
#### Types
- [x] webExtension.Extension

#### Commands
- [x] webExtension.install
- [x] webExtension.uninstall

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
