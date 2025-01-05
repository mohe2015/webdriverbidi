/// Represents the standard WebDriver BiDi events.
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum EventType {
    BrowsingContextContextCreated,
    BrowsingContextContextDestroyed,
    BrowsingContextNavigationStarted,
    BrowsingContextFragmentNavigated,
    BrowsingContextHistoryUpdated,
    BrowsingContextDomContentLoaded,
    BrowsingContextLoad,
    BrowsingContextDownloadWillBegin,
    BrowsingContextNavigationAborted,
    BrowsingContextNavigationFailed,
    BrowsingContextUserPromptClosed,
    BrowsingContextUserPromptOpened,
}

impl EventType {
    /// Parses an event type string into an EventType instance.
    pub fn from_str(event_type: &str) -> Option<Self> {
        match event_type {
            "browsingContext.ContextCreated" => Some(EventType::BrowsingContextContextCreated),
            "browsingContext.contextDestroyed" => Some(EventType::BrowsingContextContextDestroyed),
            "browsingContext.navigationStarted" => {
                Some(EventType::BrowsingContextNavigationStarted)
            }
            "browsingContext.fragmentNavigated" => {
                Some(EventType::BrowsingContextFragmentNavigated)
            }
            "browsingContext.historyUpdated" => Some(EventType::BrowsingContextHistoryUpdated),
            "browsingContext.domContentLoaded" => Some(EventType::BrowsingContextDomContentLoaded),
            "browsingContext.load" => Some(EventType::BrowsingContextLoad),
            "browsingContext.downloadWillBegin" => {
                Some(EventType::BrowsingContextDownloadWillBegin)
            }
            "browsingContext.navigationAborted" => {
                Some(EventType::BrowsingContextNavigationAborted)
            }
            "browsingContext.navigationFailed" => Some(EventType::BrowsingContextNavigationFailed),
            "browsingContext.userPromptClosed" => Some(EventType::BrowsingContextUserPromptClosed),
            "browsingContext.userPromptOpened" => Some(EventType::BrowsingContextUserPromptOpened),
            _ => None,
        }
    }
}
