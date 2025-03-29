use actix_web::{web, App, HttpServer};

use webdriverbidi::remote::browsing_context::ActivateParameters;
use webdriverbidi::remote::script::{ContextTarget, EvaluateParameters, Target};

mod utils;

// https://github.com/web-platform-tests/wpt/tree/master/webdriver/tests/bidi/browsing_context/activate
mod activate {

    use super::*;

    #[tokio::test]
    async fn test_switch_between_contexts() {
        let mut bidi_session = utils::init_session().await.unwrap();
        let top_context = utils::get_nth_context(&mut bidi_session, 0).await.unwrap();
        let new_context = utils::new_tab(&mut bidi_session).await.unwrap();

        bidi_session
            .browsing_context_activate(ActivateParameters::new(top_context.clone()))
            .await
            .unwrap();

        let initial_top_context_status =
            utils::assert_document_status(&mut bidi_session, &top_context)
                .await
                .unwrap();
        let initial_new_context_status =
            utils::assert_document_status(&mut bidi_session, &new_context)
                .await
                .unwrap();

        bidi_session
            .browsing_context_activate(ActivateParameters::new(new_context.clone()))
            .await
            .unwrap();

        let final_top_context_status =
            utils::assert_document_status(&mut bidi_session, &top_context)
                .await
                .unwrap();
        let final_new_context_status =
            utils::assert_document_status(&mut bidi_session, &new_context)
                .await
                .unwrap();

        utils::close_session(&mut bidi_session).await.unwrap();

        assert!(initial_top_context_status);
        assert!(!initial_new_context_status);

        assert!(!final_top_context_status);
        assert!(final_new_context_status);
    }

    #[tokio::test]
    async fn test_keeps_element_focused() {
        let mut bidi_session = utils::init_session().await.unwrap();

        let top_context = utils::get_nth_context(&mut bidi_session, 0).await.unwrap();

        let addr = format!("{}:0", utils::HOST);
        let server = HttpServer::new(|| {
            App::new().route(
                utils::TMP_ROUTE,
                web::get().to(utils::inline::inline_handler),
            )
        })
        .bind(addr)
        .unwrap();

        let addr = server.addrs()[0];
        let server_handle = tokio::spawn(server.run());
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let inline_url = utils::inline::build_inline(
            |path, query| format!("http://{}{}?{}", addr, path, query),
            "<textarea autofocus></textarea><input>",
            Some("html"),
            None,
            None,
            None,
        );

        let new_tab = utils::new_tab(&mut bidi_session).await.unwrap();

        utils::navigate(&mut bidi_session, new_tab.clone(), inline_url.clone())
            .await
            .unwrap();

        let params = EvaluateParameters::new(
            r#"document.querySelector("input").focus()"#.to_string(),
            Target::ContextTarget(ContextTarget::new(new_tab.clone(), None)),
            false,
            None,
            None,
            None,
        );

        bidi_session.script_evaluate(params).await.unwrap();

        let is_focused_1 = utils::is_element_focused(&mut bidi_session, new_tab.as_str(), "input")
            .await
            .unwrap();

        bidi_session
            .browsing_context_activate(ActivateParameters::new(top_context))
            .await
            .unwrap();

        let is_focused_2 = utils::is_element_focused(&mut bidi_session, new_tab.as_str(), "input")
            .await
            .unwrap();

        bidi_session
            .browsing_context_activate(ActivateParameters::new(new_tab.clone()))
            .await
            .unwrap();

        let is_focused_3 = utils::is_element_focused(&mut bidi_session, new_tab.as_str(), "input")
            .await
            .unwrap();

        utils::close_session(&mut bidi_session).await.unwrap();
        server_handle.abort();

        assert!(is_focused_1);
        assert!(is_focused_2);
        assert!(is_focused_3);
    }

    #[tokio::test]
    async fn test_multiple_activation() {
        let mut bidi_session = utils::init_session().await.unwrap();

        let addr = format!("{}:0", utils::HOST);
        let server = HttpServer::new(|| {
            App::new().route(
                utils::TMP_ROUTE,
                web::get().to(utils::inline::inline_handler),
            )
        })
        .bind(addr)
        .unwrap();

        let addr = server.addrs()[0];
        let server_handle = tokio::spawn(server.run());
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let inline_url = utils::inline::build_inline(
            |path, query| format!("http://{}{}?{}", addr, path, query),
            "<input><script>document.querySelector('input').focus();</script>",
            Some("html"),
            None,
            None,
            None,
        );

        let new_tab = utils::new_tab(&mut bidi_session).await.unwrap();
        utils::navigate(&mut bidi_session, new_tab.clone(), inline_url.clone())
            .await
            .unwrap();

        let initial_status = utils::assert_document_status(&mut bidi_session, &new_tab)
            .await
            .unwrap();
        let is_focused_1 = utils::is_element_focused(&mut bidi_session, new_tab.as_str(), "input")
            .await
            .unwrap();

        bidi_session
            .browsing_context_activate(ActivateParameters::new(new_tab.clone()))
            .await
            .unwrap();
        let middle_status = utils::assert_document_status(&mut bidi_session, &new_tab)
            .await
            .unwrap();
        let is_focused_2 = utils::is_element_focused(&mut bidi_session, new_tab.as_str(), "input")
            .await
            .unwrap();

        bidi_session
            .browsing_context_activate(ActivateParameters::new(new_tab.clone()))
            .await
            .unwrap();
        let final_status = utils::assert_document_status(&mut bidi_session, &new_tab)
            .await
            .unwrap();
        let is_focused_3 = utils::is_element_focused(&mut bidi_session, new_tab.as_str(), "input")
            .await
            .unwrap();

        utils::close_session(&mut bidi_session).await.unwrap();
        server_handle.abort();

        assert!(initial_status);
        assert!(is_focused_1);
        assert!(middle_status);
        assert!(is_focused_2);
        assert!(final_status);
        assert!(is_focused_3);
    }
}
