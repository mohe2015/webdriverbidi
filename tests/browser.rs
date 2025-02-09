use actix_web::{web, App, HttpServer};

// --------------------------------------------------

use webdriverbidi::remote::EmptyParams;

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

    use webdriverbidi::remote::browsing_context::CloseParameters;

    use super::*;
    #[tokio::test]
    async fn test_open_and_close() {
        let mut bidi_session = init_session().await.unwrap();

        let initial_windows = bidi_session
            .browser_get_client_windows(EmptyParams::new())
            .await
            .unwrap();
        assert_eq!(initial_windows.client_windows.len(), 1);

        let new_browsing_context = new_window(&mut bidi_session).await.unwrap();

        let updated_windows = bidi_session
            .browser_get_client_windows(EmptyParams::new())
            .await
            .unwrap();
        assert_eq!(updated_windows.client_windows.len(), 2);
        assert_ne!(
            updated_windows.client_windows[0].client_window,
            updated_windows.client_windows[1].client_window
        );

        bidi_session
            .browsing_context_close(CloseParameters::new(new_browsing_context, None))
            .await
            .unwrap();

        let final_windows = bidi_session
            .browser_get_client_windows(EmptyParams::new())
            .await
            .unwrap();
        assert_eq!(final_windows, initial_windows);

        close_session(&mut bidi_session).await.unwrap();
    }

    // async def test_activate_client_windows(bidi_session):
    //     initial_windows = await bidi_session.browser.get_client_windows()
    //     assert len(initial_windows) == 1
    //     initial_window = initial_windows[0]
    //     initial_window_id = initial_window["clientWindow"]

    //     initial_contexts = await bidi_session.browsing_context.get_tree()
    //     assert len(initial_contexts) == 1
    //     initial_context_id = initial_contexts[0]["context"]

    //     try:
    //         new_browsing_context = await bidi_session.browsing_context.create(type_hint="window")
    //         all_windows = await bidi_session.browser.get_client_windows()
    //         assert len(all_windows) == 2

    //         first_window = next(window for window in all_windows if window["clientWindow"] == initial_window_id)
    //         second_window = next(window for window in all_windows if window["clientWindow"] != initial_window_id)

    //         assert second_window["active"]
    //         assert not first_window["active"]

    //         await bidi_session.browsing_context.activate(context=initial_context_id)

    //         all_windows = await bidi_session.browser.get_client_windows()

    //         first_window = next(window for window in all_windows if window["clientWindow"] == initial_window_id)
    //         second_window = next(window for window in all_windows if window["clientWindow"] != initial_window_id)

    //         assert first_window["active"]
    //         assert not second_window["active"]
    //     finally:
    //         await bidi_session.browsing_context.close(context=new_browsing_context["context"])

    //     final_windows = await bidi_session.browser.get_client_windows()
    //     assert(final_windows[0]["active"]) == True
    //     assert final_windows[0]["clientWindow"] == initial_window_id
}
