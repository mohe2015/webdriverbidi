// use log::debug;
// use std::sync::Arc;

// // --------------------------------------------------

// use actix_web::{App, HttpServer, web};
// use tokio::sync::Mutex;

// // --------------------------------------------------

// use webdriverbidi::events::EventType;
// use webdriverbidi::remote::browser::RemoveUserContextParameters;
// use webdriverbidi::remote::browsing_context::{
//     ActivateParameters, CloseParameters, GetTreeParameters,
// };
// use webdriverbidi::remote::session::SubscriptionRequest;

use webdriverbidi::remote::browsing_context::ActivateParameters;

// --------------------------------------------------

mod utils;

// --------------------------------------------------

// const DEFAULT_USER_CONTEXT: &str = "default";

// --------------------------------------------------

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

    // async def test_keeps_element_focused(bidi_session, inline, new_tab, top_context):
    //     await bidi_session.browsing_context.navigate(
    //         context=new_tab["context"],
    //         url=inline("<textarea autofocus></textarea><input>"),
    //         wait="complete")

    //     await bidi_session.script.evaluate(
    //         expression="""document.querySelector("input").focus()""",
    //         target=ContextTarget(new_tab["context"]),
    //         await_promise=False)

    //     assert await is_element_focused(bidi_session, new_tab, "input")

    //     await bidi_session.browsing_context.activate(context=top_context["context"])
    //     assert await is_element_focused(bidi_session, new_tab, "input")

    //     await bidi_session.browsing_context.activate(context=new_tab["context"])
    //     assert await is_element_focused(bidi_session, new_tab, "input")

    // async def test_multiple_activation(bidi_session, inline, new_tab):
    //     await bidi_session.browsing_context.navigate(
    //         context=new_tab["context"],
    //         url=inline(
    //             "<input><script>document.querySelector('input').focus();</script>"),
    //         wait="complete")

    //     await assert_document_status(bidi_session, new_tab, visible=True, focused=True)
    //     assert await is_element_focused(bidi_session, new_tab, "input")

    //     await bidi_session.browsing_context.activate(context=new_tab["context"])
    //     await assert_document_status(bidi_session, new_tab, visible=True, focused=True)
    //     assert await is_element_focused(bidi_session, new_tab, "input")

    //     # Activate again.
    //     await bidi_session.browsing_context.activate(context=new_tab["context"])
    //     await assert_document_status(bidi_session, new_tab, visible=True, focused=True)
    //     assert await is_element_focused(bidi_session, new_tab, "input")
}
