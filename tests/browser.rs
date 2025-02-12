use actix_web::{web, App, HttpServer};

// --------------------------------------------------

mod utils;
use utils::*;

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browser-createUserContext
mod create_user_context {
    use super::*;

    #[tokio::test]
    async fn test_create_context() {
        let mut bidi_session = init_session().await.unwrap();
        let user_context = create_user_context(&mut bidi_session).await.unwrap();

        let ids = get_user_context_ids(&mut bidi_session).await.unwrap();
        assert!(ids.contains(&user_context));

        close_session(&mut bidi_session).await.unwrap();
    }

    #[tokio::test]
    async fn test_unique_id() {
        let mut bidi_session = init_session().await.unwrap();

        let first_context = create_user_context(&mut bidi_session).await.unwrap();
        let other_context = create_user_context(&mut bidi_session).await.unwrap();

        let ids = get_user_context_ids(&mut bidi_session).await.unwrap();

        assert!(ids.contains(&first_context));
        assert!(ids.contains(&other_context));

        assert_ne!(first_context, other_context);

        close_session(&mut bidi_session).await.unwrap();
    }

    #[tokio::test]
    async fn test_storage_isolation() {
        // Create and bind the server on a random port by specifying "localhost:0".
        let addr = format!("{}:0", utils::HOST);
        let server = HttpServer::new(|| {
            App::new().route(utils::TMP_ROUTE, web::get().to(inline::inline_handler))
        })
        .bind(addr)
        .unwrap();

        // Retrieve the actual address assigned.
        let addr = server.addrs()[0];

        // Spawn the server into a background task.
        let server_handle = tokio::spawn(server.run());

        // Optionally wait a brief moment for the server to be ready.
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

        let local_storage =
            get_local_storage(&mut bidi_session, tab_first_context.as_str(), test_key)
                .await
                .unwrap();
        assert_eq!(local_storage, None);

        let local_storage =
            get_local_storage(&mut bidi_session, tab_other_context.as_str(), test_key)
                .await
                .unwrap();
        assert_eq!(local_storage, None);

        set_local_storage(
            &mut bidi_session,
            tab_first_context.as_str(),
            test_key,
            test_value,
        )
        .await
        .unwrap();

        let local_storage =
            get_local_storage(&mut bidi_session, tab_first_context.as_str(), test_key)
                .await
                .unwrap();
        assert_eq!(local_storage, Some(test_value.to_string()));

        let local_storage =
            get_local_storage(&mut bidi_session, tab_other_context.as_str(), test_key)
                .await
                .unwrap();
        assert_eq!(local_storage, None);

        close_session(&mut bidi_session).await.unwrap();
        server_handle.abort();
    }
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browser-getClientWindows

mod get_client_windows {

    use webdriverbidi::remote::browsing_context::{
        ActivateParameters, CloseParameters, GetTreeParameters,
    };

    use super::*;
    #[tokio::test]
    async fn test_open_and_close() {
        let mut bidi_session = init_session().await.unwrap();

        let initial_windows = get_client_windows(&mut bidi_session).await.unwrap();
        assert_eq!(initial_windows.len(), 1);

        let new_browsing_context = new_window(&mut bidi_session).await.unwrap();

        let updated_windows = get_client_windows(&mut bidi_session).await.unwrap();
        assert_eq!(updated_windows.len(), 2);
        assert_ne!(
            updated_windows[0].client_window,
            updated_windows[1].client_window
        );

        bidi_session
            .browsing_context_close(CloseParameters::new(new_browsing_context, None))
            .await
            .unwrap();

        let final_windows = get_client_windows(&mut bidi_session).await.unwrap();
        assert_eq!(final_windows, initial_windows);

        close_session(&mut bidi_session).await.unwrap();
    }

    #[tokio::test]
    async fn test_activate_client_windows() {
        let mut bidi_session = init_session().await.unwrap();

        let initial_windows = get_client_windows(&mut bidi_session).await.unwrap();
        assert_eq!(initial_windows.len(), 1);
        let initial_window = &initial_windows[0];
        let initial_window_id = &initial_window.client_window;

        let initial_contexts = bidi_session
            .browsing_context_get_tree(GetTreeParameters::new(None, None))
            .await
            .unwrap()
            .contexts;
        assert_eq!(initial_contexts.len(), 1);
        let initial_context_id = &initial_contexts[0].context;

        let new_browsing_context = new_window(&mut bidi_session).await.unwrap();

        let all_windows = get_client_windows(&mut bidi_session).await.unwrap();
        assert_eq!(all_windows.len(), 2);

        let first_window = all_windows
            .iter()
            .find(|&window| window.client_window == *initial_window_id)
            .unwrap();
        let second_window = all_windows
            .iter()
            .find(|&window| window.client_window != *initial_window_id)
            .unwrap();
        assert!(second_window.active);
        assert!(!first_window.active);

        bidi_session
            .browsing_context_activate(ActivateParameters::new(initial_context_id.to_string()))
            .await
            .unwrap();

        let all_windows = get_client_windows(&mut bidi_session).await.unwrap();

        let first_window = all_windows
            .iter()
            .find(|&window| window.client_window == *initial_window_id)
            .unwrap();
        let second_window = all_windows
            .iter()
            .find(|&window| window.client_window != *initial_window_id)
            .unwrap();
        assert!(first_window.active);
        assert!(!second_window.active);

        bidi_session
            .browsing_context_close(CloseParameters::new(new_browsing_context, None))
            .await
            .unwrap();

        let final_windows = get_client_windows(&mut bidi_session).await.unwrap();
        assert!(final_windows[0].active);
        assert_eq!(final_windows[0].client_window, *initial_window_id);

        close_session(&mut bidi_session).await.unwrap();
    }
}
