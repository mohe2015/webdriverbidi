use serde::{Deserialize, Serialize};

// --------------------------------------------------

use super::id;
use super::utils;
use crate::define_command;
use crate::error::CommandError;
use crate::local::result_data::EmptyResult;
use crate::local::session::*;
use crate::remote::session::*;
use crate::remote::EmptyParams;
use crate::session::WebDriverBiDiSession;

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-session-status
define_command!(StatusCommand, Status, EmptyParams, status, StatusResult);

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-session-new
define_command!(NewCommand, New, NewParameters, new, NewResult);

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-session-end
define_command!(EndCommand, End, EmptyParams, end, EmptyResult);

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-session-subscribe
define_command!(
    SubscribeCommand,
    Subscribe,
    SubscriptionRequest,
    subscribe,
    SubscribeResult
);

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-session-unsubscribe
define_command!(
    UnsubscribeCommand,
    Unsubscribe,
    UnsubscribeParameters,
    unsubscribe,
    EmptyResult
);
