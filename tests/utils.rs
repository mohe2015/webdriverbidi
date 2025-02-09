use anyhow::Result;
// use base64::prelude::*;
use ctor::ctor;
use simplelog::*;
// use std::fs::File;
// use std::io::Write;
// use tokio::time;

// --------------------------------------------------

use webdriverbidi::local::script::{EvaluateResult, RemoteValue};
use webdriverbidi::remote::browsing_context::{
    CreateParameters,
    CreateType,
    NavigateParameters,
    ReadinessState,
    // GetTreeParameters,  TraverseHistoryParameters,
};
use webdriverbidi::remote::script::{
    CallFunctionParameters, ContextTarget, LocalValue, PrimitiveProtocolValue, StringValue, Target,
};
use webdriverbidi::remote::EmptyParams;
use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::webdriver::capabilities::CapabilitiesRequest;

// --------------------------------------------------

pub const HOST: &str = "localhost";
const PORT: u16 = 4444;
pub const TMP_ROUTE: &str = "/tmp.html";

// --------------------------------------------------

/// Initializes a new WebDriver BiDi.
pub async fn init_session() -> Result<WebDriverBiDiSession> {
    let capabilities = CapabilitiesRequest::default();
    let mut bidi_session = WebDriverBiDiSession::new(HOST.into(), PORT, capabilities);
    bidi_session.start().await?;
    Ok(bidi_session)
}

/// Close the WebDriver BiDi session.
pub async fn close_session(bidi_session: &mut WebDriverBiDiSession) -> Result<()> {
    bidi_session.close().await?;
    Ok(())
}

/// Returns the list of string Ids of the current user contexts.
pub async fn get_user_context_ids(bidi_session: &mut WebDriverBiDiSession) -> Result<Vec<String>> {
    let user_contexts = bidi_session
        .browser_get_user_contexts(EmptyParams::new())
        .await?
        .user_contexts;

    let user_contexts = user_contexts
        .into_iter()
        .map(|user_context_info| user_context_info.user_context)
        .collect::<Vec<String>>();

    Ok(user_contexts)
}

/// Creates a user context.
pub async fn create_user_context(bidi_session: &mut WebDriverBiDiSession) -> Result<String> {
    let user_context = bidi_session
        .browser_create_user_context(EmptyParams::new())
        .await?
        .user_context;
    Ok(user_context)
}

fn local_value(str: &str) -> LocalValue {
    LocalValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(StringValue::new(
        str.to_string(),
    )))
}

fn target_context(context: &str) -> Target {
    Target::ContextTarget(ContextTarget::new(context.to_string(), None))
}

/// Sets the value for the key in the context's localStorage.
pub async fn set_local_storage(
    bidi_session: &mut WebDriverBiDiSession,
    context: &str,
    key: &str,
    value: &str,
) -> Result<()> {
    let function_declaration = "(key, value) => localStorage.setItem(key, value)".to_string();
    let key_local_value = local_value(key);
    let value_local_value = local_value(value);
    let args = Some(vec![key_local_value, value_local_value]);
    let params = CallFunctionParameters::new(
        function_declaration,
        false,
        target_context(context),
        args,
        None,
        None,
        None,
        None,
    );
    bidi_session.script_call_function(params).await?;

    Ok(())
}

/// Returns the value identified by the key from the context's localStorage.
pub async fn get_local_storage(
    bidi_session: &mut WebDriverBiDiSession,
    context: &str,
    key: &str,
) -> Result<Option<String>> {
    let function_declaration = "(key) => localStorage.getItem(key)".to_string();
    let key_local_value = local_value(key);

    let args = Some(vec![key_local_value]);
    let params = CallFunctionParameters::new(
        function_declaration,
        false,
        target_context(context),
        args,
        None,
        None,
        None,
        None,
    );
    let eval_result = bidi_session.script_call_function(params).await?;

    match eval_result {
        EvaluateResult::EvaluateResultSuccess(eval_rslt_success) => {
            let remote_value = eval_rslt_success.result;
            match remote_value {
                RemoteValue::PrimitiveProtocolValue(
                    webdriverbidi::local::script::PrimitiveProtocolValue::StringValue(string_value),
                ) => Ok(Some(string_value.value)),
                _ => Ok(None),
            }
        }
        _ => Ok(None),
    }
}

// --------------------------------------------------

// /// Sleep for a given number of seconds.
// pub async fn sleep_for_secs(secs: u64) {
//     time::sleep(time::Duration::from_secs(secs)).await
// }

// // --------------------------------------------------

// /// Save a Base64-encoded screenshot to a file.
// pub fn save_screenshot(base64_data: &str, file_path: &str) -> std::io::Result<()> {
//     // Decode the Base64 string into bytes
//     let decoded_data = BASE64_STANDARD
//         .decode(base64_data)
//         .expect("Failed to decode Base64 data");

//     // Create a new file and write the decoded bytes
//     let mut file = File::create(file_path)?;
//     file.write_all(&decoded_data)?;

//     println!("Screenshot saved to {}", file_path);
//     Ok(())
// }

// // --------------------------------------------------

// /// Get the first browsing context from the browsing context tree.
// pub async fn get_first_context(
//     session: &mut WebDriverBiDiSession,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let get_tree_params = GetTreeParameters::new(None, None);
//     let get_tree_rslt = session.browsing_context_get_tree(get_tree_params).await?;
//     Ok(get_tree_rslt.contexts[0].context.clone())
// }

// // --------------------------------------------------

/// Open a new tab.
pub async fn new_tab_in_user_context(
    session: &mut WebDriverBiDiSession,
    user_context: String,
) -> Result<String> {
    let create_params = CreateParameters::new(CreateType::Tab, None, None, Some(user_context));
    let context = session
        .browsing_context_create(create_params)
        .await?
        .context;
    Ok(context)
}

/// Open a new tab.
pub async fn new_window(
    session: &mut WebDriverBiDiSession,
) -> Result<String> {
    let create_params = CreateParameters::new(CreateType::Window, None, None, None);
    let context = session
        .browsing_context_create(create_params)
        .await?
        .context;
    Ok(context)
}
// // --------------------------------------------------

/// Initialize a simplelog TermLogger.
#[ctor]
fn init() {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
}

// // --------------------------------------------------

/// Navigates to the specified URL waiting for the document to completely load.
pub async fn navigate(
    session: &mut WebDriverBiDiSession,
    context: String,
    url: String,
) -> Result<()> {
    let navigate_params = NavigateParameters::new(context, url, Some(ReadinessState::Complete));
    session.browsing_context_navigate(navigate_params).await?;
    Ok(())
}

// // --------------------------------------------------

// pub async fn traverse_history(session: &mut WebDriverBiDiSession, context: String, delta: i64) {
//     let traverse_history_params = TraverseHistoryParameters::new(context.clone(), delta);
//     session
//         .browsing_context_traverse_history(traverse_history_params)
//         .await
//         .expect("Failed to send command");
// }

pub mod inline {
    use actix_web::{web, HttpResponse, Responder};
    use serde::Deserialize;
    use std::collections::HashMap;
    use url::form_urlencoded;

    /// A helper function that “inlines” a document by wrapping the given source
    /// in a boilerplate template and then building a URL with the resulting document
    /// embedded as a query parameter.
    ///
    /// The `build_url` parameter is a callback that takes a path and a query string
    /// and returns a URL.
    pub fn build_inline<F>(
        build_url: F,
        src: &str,
        doctype: Option<&str>,
        mime: Option<&str>,
        charset: Option<&str>,
        parameters: Option<HashMap<String, String>>,
    ) -> String
    where
        F: Fn(&str, &str) -> String,
    {
        // Set default values.
        let doctype = doctype.unwrap_or("html");
        let mime = mime.unwrap_or(match doctype {
            "html" | "html_quirks" => "text/html",
            "xhtml" => "application/xhtml+xml",
            "xml" => "text/xml",
            "js" => "text/javascript",
            _ => "text/html",
        });
        let charset = charset.unwrap_or("UTF-8");

        // Choose the appropriate boilerplate.
        let template = match doctype {
            "html" => "<!doctype html>\n<meta charset={charset}>\n{src}",
            "html_quirks" => "{src}",
            "xhtml" => {
                r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN"
        "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">
    <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en" lang="en">
      <head>
        <title>XHTML might be the future</title>
      </head>

      <body>
        {src}
      </body>
    </html>"#
            }
            "xml" => r#"<?xml version="1.0" encoding="{charset}"?>\n{src}"#,
            "js" => "",
            _ => "",
        };

        // Replace placeholders with provided values.
        let doc = template.replace("{charset}", charset).replace("{src}", src);

        // Build the query string with the required parameters.
        let mut serializer = form_urlencoded::Serializer::new(String::new());
        serializer.append_pair("doc", &doc);
        serializer.append_pair("mime", mime);
        serializer.append_pair("charset", charset);
        if let Some(params) = parameters {
            for (key, value) in params {
                serializer.append_pair(&key, &value);
            }
        }
        let query = serializer.finish();

        // Use the provided callback to build the full URL.
        build_url(super::TMP_ROUTE, &query)
    }

    /// This struct is used to deserialize the query parameters passed to our handler.
    #[derive(Deserialize)]
    pub struct InlineQuery {
        doc: Option<String>,
        mime: Option<String>,
        charset: Option<String>,
    }

    /// This function is our request handler. It mimics the Python `main()` function
    /// by reading the query parameters (looking for “doc”, “mime”, and “charset”),
    /// constructing a `Content-Type` header if appropriate, and returning the document
    /// or an error message if “doc” is missing.
    pub async fn inline_handler(query: web::Query<InlineQuery>) -> impl Responder {
        // If the "doc" parameter is missing, return a 404 error.
        if query.doc.is_none() {
            return HttpResponse::NotFound()
                .content_type("text/plain")
                .body("Missing doc parameter in query");
        }
        let doc = query.doc.clone().unwrap();

        // Build a content-type header (e.g. "text/html;charset=UTF-8") if provided.
        let mut content_type_parts = Vec::new();
        if let Some(mime) = &query.mime {
            content_type_parts.push(mime.clone());
        }
        if let Some(charset) = &query.charset {
            content_type_parts.push(format!("charset={}", charset));
        }

        let mut response = HttpResponse::Ok();
        response.insert_header(("X-XSS-Protection", "0"));
        if !content_type_parts.is_empty() {
            let content_type_header = content_type_parts.join(";");
            response.content_type(content_type_header);
        }
        response.body(doc)
    }
}
