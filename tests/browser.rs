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
