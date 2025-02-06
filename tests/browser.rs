use webdriverbidi::remote::EmptyParams;

// --------------------------------------------------

mod utils;
use utils::*;

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browser-createUserContext

#[tokio::test]
async fn test_create_context() {
    let mut bidi_session = init_session().await.unwrap();

    let user_context = bidi_session
        .browser_create_user_context(EmptyParams::new())
        .await
        .unwrap()
        .user_context;

    let ids = get_user_context_ids(&mut bidi_session).await.unwrap();
    close_session(&mut bidi_session).await.unwrap();

    assert!(ids.contains(&user_context));
}
