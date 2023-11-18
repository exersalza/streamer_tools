use std::borrow::Cow;
use std::net::SocketAddr;
use std::ops::ControlFlow;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    headers,
    response::IntoResponse, TypedHeader,
};
//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;
//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    log::info!("Useragent {}, at {}", user_agent, addr);

    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

// don't touch my clogs

// thanks to the guy that wrote the example in axum/examples
async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    // send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        log::info!("Pinged {who}...");
    } else {
        log::info!("Could not send ping {who}!");
        return;
    }

    let (mut tx, _rx) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    tokio::spawn(async move {
        loop {
            // In case of any websocket error, we exit.
            if tx.send(Message::Text(format!("inc"))).await.is_err() {
                break;
            }

            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }

        log::info!("Sending close to {who}...");
        if let Err(e) = tx
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from("Goodbye"),
            })))
            .await
        {
            println!("Could not send Close due to {e}, probably it is ok?");
        }
    });
}

fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {who} sent str: {t:?}");
        }
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {who} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            println!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
