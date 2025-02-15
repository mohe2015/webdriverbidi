use actix_web::{web, App, HttpServer};

// --------------------------------------------------

use webdriverbidi::remote::browsing_context::{
    ActivateParameters, CloseParameters, GetTreeParameters,
};

// --------------------------------------------------

mod utils;
use utils::*;

// --------------------------------------------------

const DEFAULT_USER_CONTEXT: &str = "default";

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

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browser-getUserContexts

mod get_user_contexts {
    use super::*;

    #[tokio::test]
    async fn test_default() {
        let mut bidi_session = init_session().await.unwrap();

        let user_context_ids = get_user_context_ids(&mut bidi_session).await.unwrap();

        assert!(!user_context_ids.is_empty());
        assert!(user_context_ids.contains(&DEFAULT_USER_CONTEXT.to_string()));

        close_session(&mut bidi_session).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_remove_contexts() {
        let mut bidi_session = init_session().await.unwrap();

        // create two user contexts
        let user_context_1 = create_user_context(&mut bidi_session).await.unwrap();
        let user_context_2 = create_user_context(&mut bidi_session).await.unwrap();

        // get_user_contexts should return at least 3 contexts:
        // the default context and the 2 newly created contexts
        let user_context_ids = get_user_context_ids(&mut bidi_session).await.unwrap();
        assert!(user_context_ids.len() >= 3);
        assert!(user_context_ids.contains(&user_context_1));
        assert!(user_context_ids.contains(&user_context_2));
        assert!(user_context_ids.contains(&DEFAULT_USER_CONTEXT.to_string()));

        // remove user context 1
        remove_user_context(&mut bidi_session, user_context_1.clone())
            .await
            .unwrap();

        // assert that user context 1 is not returned by browser.getUserContexts
        let user_context_ids = get_user_context_ids(&mut bidi_session).await.unwrap();
        assert!(!user_context_ids.contains(&user_context_1));
        assert!(user_context_ids.contains(&user_context_2));
        assert!(user_context_ids.contains(&DEFAULT_USER_CONTEXT.to_string()));

        // remove user context 2
        remove_user_context(&mut bidi_session, user_context_2.clone())
            .await
            .unwrap();

        // assert that user context 2 is not returned by browser.getUserContexts
        let user_context_ids = get_user_context_ids(&mut bidi_session).await.unwrap();
        assert!(!user_context_ids.contains(&user_context_2));
        assert!(user_context_ids.contains(&DEFAULT_USER_CONTEXT.to_string()));

        close_session(&mut bidi_session).await.unwrap();
    }
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browser-removeUserContext

mod remove_user_context {
    use super::*;

    const USER_PROMPT_OPENED_EVENT: &str = "browsingContext.userPromptOpened";

    #[tokio::test]
    async fn test_remove_context() {
        let mut bidi_session = init_session().await.unwrap();

        let user_context = create_user_context(&mut bidi_session).await.unwrap();
        let user_context_ids = get_user_context_ids(&mut bidi_session).await.unwrap();
        assert!(user_context_ids.contains(&user_context));

        remove_user_context(&mut bidi_session, user_context.clone())
            .await
            .unwrap();

        let user_context_ids = get_user_context_ids(&mut bidi_session).await.unwrap();
        assert!(!user_context_ids.contains(&user_context));
        assert!(user_context_ids.contains(&DEFAULT_USER_CONTEXT.to_string()));

        close_session(&mut bidi_session).await.unwrap();
    }

    // @pytest.mark.parametrize("type_hint", ["tab", "window"])
    // @pytest.mark.asyncio
    // async def test_remove_context_closes_contexts(
    //     bidi_session, subscribe_events, create_user_context, type_hint
    // ):
    //     await subscribe_events(events=["browsingContext.contextDestroyed"])

    //     user_context_1 = await create_user_context()
    //     user_context_2 = await create_user_context()

    //     # context 1 and 2 are owned by user context 1
    //     context_1 = await bidi_session.browsing_context.create(
    //         user_context=user_context_1, type_hint=type_hint
    //     )
    //     context_2 = await bidi_session.browsing_context.create(
    //         user_context=user_context_1, type_hint=type_hint
    //     )
    //     # context 3 and 4 are owned by user context 2
    //     context_3 = await bidi_session.browsing_context.create(
    //         user_context=user_context_2, type_hint=type_hint
    //     )
    //     context_4 = await bidi_session.browsing_context.create(
    //         user_context=user_context_2, type_hint=type_hint
    //     )

    //     # Track all received browsingContext.contextDestroyed events in the events array
    //     events = []

    //     async def on_event(method, data):
    //         events.append(data)

    //     remove_listener = bidi_session.add_event_listener("browsingContext.contextDestroyed", on_event)

    //     # destroy user context 1 and wait for context 1 and 2 to be destroyed
    //     await bidi_session.browser.remove_user_context(user_context=user_context_1)

    //     wait = AsyncPoll(bidi_session, timeout=2)
    //     await wait.until(lambda _: len(events) >= 2)

    //     assert len(events) == 2
    //     destroyed_contexts = [event["context"] for event in events]
    //     assert context_1["context"] in destroyed_contexts
    //     assert context_2["context"] in destroyed_contexts

    //     # destroy user context 1 and wait for context 3 and 4 to be destroyed
    //     await bidi_session.browser.remove_user_context(user_context=user_context_2)

    //     wait = AsyncPoll(bidi_session, timeout=2)
    //     await wait.until(lambda _: len(events) >= 4)

    //     assert len(events) == 4
    //     destroyed_contexts = [event["context"] for event in events]
    //     assert context_3["context"] in destroyed_contexts
    //     assert context_4["context"] in destroyed_contexts

    //     remove_listener()

    // @pytest.mark.parametrize("type_hint", ["tab", "window"])
    // @pytest.mark.asyncio
    // async def test_remove_context_skips_beforeunload_prompt(
    //     bidi_session,
    //     subscribe_events,
    //     create_user_context,
    //     setup_beforeunload_page,
    //     type_hint,
    // ):
    //     await subscribe_events(events=[USER_PROMPT_OPENED_EVENT])

    //     events = []

    //     async def on_event(method, data):
    //         if data["type"] == "beforeunload":
    //             events.append(method)

    //     remove_listener = bidi_session.add_event_listener(
    //         USER_PROMPT_OPENED_EVENT, on_event)

    //     user_context = await create_user_context()

    //     context = await bidi_session.browsing_context.create(
    //         user_context=user_context, type_hint=type_hint
    //     )

    //     await setup_beforeunload_page(context)

    //     await bidi_session.browser.remove_user_context(user_context=user_context)

    //     wait = AsyncPoll(bidi_session, timeout=0.5)
    //     with pytest.raises(TimeoutException):
    //         await wait.until(lambda _: len(events) > 0)

    //     remove_listener()
}
