use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:4252".to_string();
    let listener = TcpListener::bind(&addr).await?;

    while let Ok((stream)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await?;
    println!("WebSocket connection established");

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() {
            let received_text = msg.to_text()?;
            println!("Received message: {}", received_text);
            ws_stream.send(Message::Text(received_text.to_string())).await?;
        }
    }

    Ok(())
}
