use super::{requests::*, *};

use httptest::{matchers::*, responders::*, Expectation, Server};
use serde_json::json;

static TOKEN: &str = "abc123";

#[test]
fn test_file_type() {
    let url = FileType::URL("test".into());
    assert_eq!(url.needs_upload(), false, "url does not need upload");
    assert!(url.file().is_none(), "url does not have file");

    let file_id = FileType::FileID("test".into());
    assert_eq!(
        file_id.needs_upload(),
        false,
        "file_id does not need upload"
    );
    assert!(file_id.file().is_none(), "file_id does not have file");

    let attach = FileType::Attach("test".into());
    assert_eq!(attach.needs_upload(), false, "attach does not need upload");
    assert!(attach.file().is_none(), "attach does not have file");

    let bytes = FileType::Bytes("name".into(), vec![1, 2, 3]);
    assert_eq!(bytes.needs_upload(), true, "bytes needs upload");
    assert!(bytes.file().is_some(), "bytes has file");
}

#[test]
fn test_input_media() {
    let photo = InputMedia::Photo(InputMediaPhoto {
        caption: Some("caption".into()),
        media: FileType::FileID("test1".into()),
        ..Default::default()
    });
    let new_photo = photo.update_media(FileType::URL("test2".into()));
    assert_eq!(new_photo.get_file(), &FileType::URL("test2".into()));

    let input_media_photo = match new_photo {
        InputMedia::Photo(photo) => photo,
        _ => panic!("input media had wrong type"),
    };
    assert_eq!(
        input_media_photo.caption.unwrap(),
        "caption",
        "caption should be the same"
    );

    let video = InputMedia::Video(InputMediaVideo {
        caption: Some("caption".into()),
        media: FileType::FileID("test1".into()),
        ..Default::default()
    });
    let new_video = video.update_media(FileType::URL("test2".into()));
    assert_eq!(new_video.get_file(), &FileType::URL("test2".into()));

    let input_media_video = match new_video {
        InputMedia::Video(video) => video,
        _ => panic!("input media had wrong type"),
    };
    assert_eq!(
        input_media_video.caption.unwrap(),
        "caption",
        "caption should be the same"
    );
}

#[tokio::test]
async fn test_download_file() {
    let _ = pretty_env_logger::try_init();

    let file_id = "123";
    let body = "helloworld".to_string();

    let server = Server::run();
    server.expect(
        Expectation::matching(all_of![
            request::method("GET"),
            request::path(format!("/file/bot{}/{}", TOKEN, file_id)),
        ])
        .respond_with(status_code(200).body("helloworld")),
    );

    let telegram = Telegram::new_with_endpoint(TOKEN.into(), server.url("").to_string());
    let file = telegram.download_file(file_id).await;

    assert!(file.is_ok(), "correct response was not ok");
    assert_eq!(body.into_bytes(), file.unwrap(), "body was not correct");
}

#[tokio::test]
async fn test_webhook() -> failure::Fallible<()> {
    let _ = pretty_env_logger::try_init();

    let endpoint = "http://example.com";

    let server = Server::run();
    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path(format!("/bot{}/setWebhook", TOKEN)),
            request::body(json_decoded(eq(json!({ "url": endpoint }))))
        ])
        .respond_with(json_encoded(json!({"ok": true, "result": true}))),
    );
    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path(format!("/bot{}/deleteWebhook", TOKEN)),
            request::body(json_decoded(eq(json!(serde_json::Value::Null))))
        ])
        .respond_with(json_encoded(json!({"ok": true, "result": true}))),
    );

    let telegram = Telegram::new_with_endpoint(TOKEN.into(), server.url("").to_string());

    let set_webhook = SetWebhook {
        url: endpoint.into(),
    };
    let resp = telegram.make_request(&set_webhook).await?;
    assert_eq!(resp, true);

    let delete_webhook = DeleteWebhook;
    let resp = telegram.make_request(&delete_webhook).await?;
    assert_eq!(resp, true);

    Ok(())
}

#[tokio::test]
async fn test_get_me() -> failure::Fallible<()> {
    let user = User {
        id: 123,
        first_name: "Test".into(),
        username: Some("test_bot".into()),
        last_name: None,
        is_bot: true,
        language_code: None,
    };

    let server = Server::run();
    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path(format!("/bot{}/getMe", TOKEN)),
            request::body(json_decoded(eq(json!(serde_json::Value::Null))))
        ])
        .respond_with(json_encoded(json!({
            "ok": true,
            "result": {
                "id": user.id,
                "first_name": user.first_name,
                "username": user.username,
                "is_bot": user.is_bot
            }
        }))),
    );

    let telegram = Telegram::new_with_endpoint(TOKEN.into(), server.url("").to_string());
    let get_me = GetMe;
    let resp = telegram.make_request(&get_me).await?;

    assert_eq!(resp, user, "user information must be the same");

    Ok(())
}
