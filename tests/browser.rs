use log::debug;
use std::sync::Arc;

// --------------------------------------------------

use actix_files::NamedFile;
use actix_web::{web, App, HttpServer};
use tokio::sync::Mutex;

// --------------------------------------------------

use webdriverbidi::events::EventType;
use webdriverbidi::remote::browser::RemoveUserContextParameters;
use webdriverbidi::remote::browsing_context::{
    ActivateParameters, CloseParameters, GetTreeParameters,
};
use webdriverbidi::remote::session::SubscriptionRequest;

// --------------------------------------------------

mod utils;
use utils::*;

// --------------------------------------------------

const DEFAULT_USER_CONTEXT: &str = "default";

// --------------------------------------------------

// https://github.com/web-platform-tests/wpt/tree/master/webdriver/tests/bidi/browser/create_user_context
mod create_user_context {
    use super::*;

    #[tokio::test]
    async fn test_create_context() {
        let mut bidi_session = init_session().await.unwrap();

        let user_context = create_user_context(&mut bidi_session).await.unwrap();
        let ids = get_user_context_ids(&mut bidi_session).await.unwrap();

        close_session(&mut bidi_session).await.unwrap();

        assert!(ids.contains(&user_context));
    }

    #[tokio::test]
    async fn test_unique_id() {
        let mut bidi_session = init_session().await.unwrap();

        let first_context = create_user_context(&mut bidi_session).await.unwrap();
        let other_context = create_user_context(&mut bidi_session).await.unwrap();
        let ids = get_user_context_ids(&mut bidi_session).await.unwrap();

        close_session(&mut bidi_session).await.unwrap();

        assert!(ids.contains(&first_context));
        assert!(ids.contains(&other_context));
        assert_ne!(first_context, other_context);
    }

    #[tokio::test]
    async fn test_storage_isolation() {
        let addr = format!("{}:0", utils::HOST);
        let server = HttpServer::new(|| {
            App::new().route(utils::TMP_ROUTE, web::get().to(inline::inline_handler))
        })
        .bind(addr)
        .unwrap();

        let addr = server.addrs()[0];
        let server_handle = tokio::spawn(server.run());
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let inline_url = inline::build_inline(
            |path, query| format!("http://{}{}?{}", addr, path, query),
            "<p>test</p>",
            Some("html"),
            None,
            None,
            None,
        );

        let mut bidi_session = init_session().await.unwrap();

        let first_context = create_user_context(&mut bidi_session).await.unwrap();
        let other_context = create_user_context(&mut bidi_session).await.unwrap();

        let tab_first_context = new_tab_in_user_context(&mut bidi_session, first_context)
            .await
            .unwrap();
        let tab_other_context = new_tab_in_user_context(&mut bidi_session, other_context)
            .await
            .unwrap();

        navigate(
            &mut bidi_session,
            tab_first_context.clone(),
            inline_url.clone(),
        )
        .await
        .unwrap();

        navigate(
            &mut bidi_session,
            tab_other_context.clone(),
            inline_url.clone(),
        )
        .await
        .unwrap();

        let test_key = "test";
        let test_value = "value";

        let initial_tab_first_context_storage =
            get_local_storage(&mut bidi_session, tab_first_context.as_str(), test_key)
                .await
                .unwrap();

        let initial_tab_other_context_storage =
            get_local_storage(&mut bidi_session, tab_other_context.as_str(), test_key)
                .await
                .unwrap();

        set_local_storage(
            &mut bidi_session,
            tab_first_context.as_str(),
            test_key,
            test_value,
        )
        .await
        .unwrap();

        let final_tab_first_storage =
            get_local_storage(&mut bidi_session, tab_first_context.as_str(), test_key)
                .await
                .unwrap();

        let final_tab_other_storage =
            get_local_storage(&mut bidi_session, tab_other_context.as_str(), test_key)
                .await
                .unwrap();

        close_session(&mut bidi_session).await.unwrap();
        server_handle.abort();

        assert_eq!(initial_tab_first_context_storage, None);
        assert_eq!(initial_tab_other_context_storage, None);
        assert_eq!(final_tab_first_storage, Some(test_value.to_string()));
        assert_eq!(final_tab_other_storage, None);
    }
}

// --------------------------------------------------

// https://github.com/web-platform-tests/wpt/tree/master/webdriver/tests/bidi/browser/get_client_windows
mod get_client_windows {

    use super::*;

    #[tokio::test]
    async fn test_open_and_close() {
        let mut bidi_session = init_session().await.unwrap();

        let initial_windows = get_client_windows(&mut bidi_session).await.unwrap();

        let new_browsing_context = new_window(&mut bidi_session).await.unwrap();

        let updated_windows = get_client_windows(&mut bidi_session).await.unwrap();

        bidi_session
            .browsing_context_close(CloseParameters::new(new_browsing_context, None))
            .await
            .unwrap();

        let final_windows = get_client_windows(&mut bidi_session).await.unwrap();
        close_session(&mut bidi_session).await.unwrap();

        assert_eq!(initial_windows.len(), 1);
        assert_eq!(updated_windows.len(), 2);
        assert_ne!(
            updated_windows[0].client_window,
            updated_windows[1].client_window
        );
        assert_eq!(final_windows, initial_windows);
    }

    #[tokio::test]
    async fn test_activate_client_windows() {
        let mut bidi_session = init_session().await.unwrap();

        let initial_windows = get_client_windows(&mut bidi_session).await.unwrap();
        let initial_window = &initial_windows[0];
        let initial_window_id = &initial_window.client_window;

        let initial_contexts = bidi_session
            .browsing_context_get_tree(GetTreeParameters::new(None, None))
            .await
            .unwrap()
            .contexts;

        let initial_context_id = &initial_contexts[0].context;

        let new_browsing_context = new_window(&mut bidi_session).await.unwrap();

        let initial_all_windows = get_client_windows(&mut bidi_session).await.unwrap();

        let initial_first_window = initial_all_windows
            .iter()
            .find(|&window| window.client_window == *initial_window_id)
            .unwrap();

        let initial_second_window = initial_all_windows
            .iter()
            .find(|&window| window.client_window != *initial_window_id)
            .unwrap();

        bidi_session
            .browsing_context_activate(ActivateParameters::new(initial_context_id.to_string()))
            .await
            .unwrap();

        let final_all_windows = get_client_windows(&mut bidi_session).await.unwrap();

        let final_first_window = final_all_windows
            .iter()
            .find(|&window| window.client_window == *initial_window_id)
            .unwrap();
        let final_second_window = final_all_windows
            .iter()
            .find(|&window| window.client_window != *initial_window_id)
            .unwrap();

        bidi_session
            .browsing_context_close(CloseParameters::new(new_browsing_context, None))
            .await
            .unwrap();

        let final_windows = get_client_windows(&mut bidi_session).await.unwrap();

        close_session(&mut bidi_session).await.unwrap();

        assert_eq!(initial_windows.len(), 1);
        assert_eq!(initial_contexts.len(), 1);

        assert_eq!(initial_all_windows.len(), 2);
        assert!(initial_second_window.active);
        assert!(!initial_first_window.active);

        assert!(final_first_window.active);
        assert!(!final_second_window.active);

        assert!(final_windows[0].active);
        assert_eq!(final_windows[0].client_window, *initial_window_id);
    }
}

// --------------------------------------------------

// https://github.com/web-platform-tests/wpt/tree/master/webdriver/tests/bidi/browser/get_user_contexts
mod get_user_contexts {
    use super::*;

    #[tokio::test]
    async fn test_default() {
        let mut bidi_session = init_session().await.unwrap();

        let user_context_ids = get_user_context_ids(&mut bidi_session).await.unwrap();

        close_session(&mut bidi_session).await.unwrap();

        assert!(!user_context_ids.is_empty());
        assert!(user_context_ids.contains(&DEFAULT_USER_CONTEXT.to_string()));
    }

    #[tokio::test]
    async fn test_create_remove_contexts() {
        let mut bidi_session = init_session().await.unwrap();

        let user_context_1 = create_user_context(&mut bidi_session).await.unwrap();
        let user_context_2 = create_user_context(&mut bidi_session).await.unwrap();
        let user_context_ids_1 = get_user_context_ids(&mut bidi_session).await.unwrap();

        remove_user_context(&mut bidi_session, user_context_1.clone())
            .await
            .unwrap();

        let user_context_ids_2 = get_user_context_ids(&mut bidi_session).await.unwrap();

        remove_user_context(&mut bidi_session, user_context_2.clone())
            .await
            .unwrap();

        let user_context_ids_3 = get_user_context_ids(&mut bidi_session).await.unwrap();

        close_session(&mut bidi_session).await.unwrap();

        assert!(user_context_ids_1.len() >= 3);
        assert!(user_context_ids_1.contains(&user_context_1));
        assert!(user_context_ids_1.contains(&user_context_2));
        assert!(user_context_ids_1.contains(&DEFAULT_USER_CONTEXT.to_string()));

        assert!(!user_context_ids_2.contains(&user_context_1));
        assert!(user_context_ids_2.contains(&user_context_2));
        assert!(user_context_ids_2.contains(&DEFAULT_USER_CONTEXT.to_string()));

        assert!(!user_context_ids_3.contains(&user_context_2));
        assert!(user_context_ids_3.contains(&DEFAULT_USER_CONTEXT.to_string()));
    }
}

// --------------------------------------------------

// https://github.com/web-platform-tests/wpt/tree/master/webdriver/tests/bidi/browser/remove_user_context
mod remove_user_context {
    use super::*;

    const BROWSING_CTX_DESTROYED_EVENT: &str = "browsingContext.contextDestroyed";
    const USER_PROMPT_OPENED_EVENT: &str = "browsingContext.userPromptOpened";

    async fn before_unload() -> NamedFile {
        utils::serve_static_html("./static/beforeunload.html").await
    }

    #[tokio::test]
    async fn test_remove_context() {
        let mut bidi_session = init_session().await.unwrap();

        let user_context = create_user_context(&mut bidi_session).await.unwrap();
        let initial_user_context_ids = get_user_context_ids(&mut bidi_session).await.unwrap();

        remove_user_context(&mut bidi_session, user_context.clone())
            .await
            .unwrap();

        let final_user_context_ids = get_user_context_ids(&mut bidi_session).await.unwrap();

        close_session(&mut bidi_session).await.unwrap();

        assert!(initial_user_context_ids.contains(&user_context));

        assert!(!final_user_context_ids.contains(&user_context));
        assert!(final_user_context_ids.contains(&DEFAULT_USER_CONTEXT.to_string()));
    }

    #[tokio::test]
    async fn test_remove_context_closes_contexts() {
        let mut bidi_session = init_session().await.unwrap();

        let events = Arc::new(Mutex::new(Vec::<serde_json::Value>::new()));

        {
            let events = events.clone();

            bidi_session
                .register_event_handler(
                    EventType::BrowsingContextContextDestroyed,
                    move |event: serde_json::Value| {
                        let events = events.clone();
                        async move {
                            debug!(
                                "Received browsingContext.contextDestroyed event: {:?}",
                                event
                            );
                            events.lock().await.push(event);
                        }
                    },
                )
                .await;
        }

        bidi_session
            .session_subscribe(SubscriptionRequest::new(
                vec![String::from(BROWSING_CTX_DESTROYED_EVENT)],
                None,
                None,
            ))
            .await
            .unwrap();

        let user_context_1 = create_user_context(&mut bidi_session).await.unwrap();
        let user_context_2 = create_user_context(&mut bidi_session).await.unwrap();

        let context_1 = new_tab_in_user_context(&mut bidi_session, user_context_1.clone())
            .await
            .unwrap();

        let context_2 = new_tab_in_user_context(&mut bidi_session, user_context_1.clone())
            .await
            .unwrap();

        let context_3 = new_tab_in_user_context(&mut bidi_session, user_context_2.clone())
            .await
            .unwrap();

        let context_4 = new_tab_in_user_context(&mut bidi_session, user_context_2.clone())
            .await
            .unwrap();

        bidi_session
            .browser_remove_user_context(RemoveUserContextParameters::new(user_context_1))
            .await
            .unwrap();

        let initial_events_len = events.lock().await.len();

        let initial_destroyed_contexts = events
            .lock()
            .await
            .iter()
            .map(|event| {
                event.clone()["params"]["context"]
                    .as_str()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<_>>();

        bidi_session
            .browser_remove_user_context(RemoveUserContextParameters::new(user_context_2))
            .await
            .unwrap();

        close_session(&mut bidi_session).await.unwrap();

        let final_events_len = events.lock().await.len();

        let final_destroyed_contexts = events
            .lock()
            .await
            .iter()
            .map(|event| {
                event.clone()["params"]["context"]
                    .as_str()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<_>>();

        assert!(initial_events_len == 2);

        assert!(initial_destroyed_contexts.contains(&context_1));
        assert!(initial_destroyed_contexts.contains(&context_2));

        assert!(final_events_len == 4);

        assert!(final_destroyed_contexts.contains(&context_3));
        assert!(final_destroyed_contexts.contains(&context_4));
    }

    #[tokio::test]
    async fn test_remove_context_skips_beforeunload_prompt() {
        let mut bidi_session = init_session().await.unwrap();

        bidi_session
            .session_subscribe(SubscriptionRequest::new(
                vec![String::from(USER_PROMPT_OPENED_EVENT)],
                None,
                None,
            ))
            .await
            .unwrap();

        let events = Arc::new(Mutex::new(Vec::<serde_json::Value>::new()));

        {
            let events = events.clone();

            bidi_session
                .register_event_handler(
                    EventType::BrowsingContextUserPromptOpened,
                    move |event: serde_json::Value| {
                        let events = events.clone();
                        async move {
                            debug!(
                                "Received browsingContext.userPromptOpened event: {:?}",
                                event
                            );
                            events.lock().await.push(event);
                        }
                    },
                )
                .await;
        }

        let addr = format!("{}:0", utils::HOST);

        let server = HttpServer::new(|| App::new().route("/", web::get().to(before_unload)))
            .bind(addr)
            .unwrap();

        let addr = server.addrs()[0];
        let url = format!("http://{}", addr);
        let server_handle = tokio::spawn(server.run());
        utils::sleep_for_millis(100).await;

        let user_context = create_user_context(&mut bidi_session).await.unwrap();
        let context = new_tab_in_user_context(&mut bidi_session, user_context.clone())
            .await
            .unwrap();

        navigate(&mut bidi_session, context, url).await.unwrap();

        remove_user_context(&mut bidi_session, user_context)
            .await
            .unwrap();

        close_session(&mut bidi_session).await.unwrap();
        server_handle.abort();

        // debug!("Events vec len: {}", events.lock().await.len());
        assert!(events.lock().await.len() > 0);
    }
}
