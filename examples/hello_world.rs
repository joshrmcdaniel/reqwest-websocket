use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest_impersonate::Client;
use reqwest_websocket::{Error, Message, RequestBuilderExt};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let websocket = Client::default()
        .get("wss://echo.websocket.org/")
        .upgrade()
        .send()
        .await?
        .into_websocket()
        .await?;

    let (mut tx, mut rx) = websocket.split();

    futures_util::future::join(
        async move {
            for i in 1..11 {
                tx.send(format!("Hello, World! #{i}").into()).await.unwrap();
            }
        },
        async move {
            while let Some(message) = rx.try_next().await.unwrap() {
                if let Message::Text(text) = message {
                    println!("received: {text}");
                }
            }
        },
    )
    .await;

    Ok(())
}
