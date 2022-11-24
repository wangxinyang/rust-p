mod msg;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};

pub use msg::{Msg, MsgData};

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket);
}

async fn handle_socket(mut socket: WebSocket) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if let Message::Text(msg) = msg {
                println!("client sent str: {:?}", msg);
            }
        } else {
            println!("client disconnected");
        }
    }

    // loop {
    //     if socket
    //         .send(Message::Text(String::from("Hi!")))
    //         .await
    //         .is_err()
    //     {
    //         println!("client disconnected");
    //         return;
    //     }
    //     tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    // }
}
