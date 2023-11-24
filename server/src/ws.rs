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
use axum::extract::Path;
use std::sync::Mutex;
use axum::extract::ws::CloseFrame;
//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};
use lazy_static::lazy_static;
use rand::random;
use frontend::components::timer::{Time, Timer};
use crate::SQL;


lazy_static! {
    static ref sub_counter: Mutex<u8> = Mutex::new(0);
}

pub async fn ws_handler(Path(t): Path<String>,
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

    ws.on_upgrade(move |socket| handle_socket(socket, addr, t))
}

// don't touch my clogs

// thanks to the guy that wrote the example in axum/examples
async fn handle_socket(mut socket: WebSocket, who: SocketAddr, _type: String) {
    // send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        log::info!("Pinged {who}...");
    } else {
        log::info!("Could not send ping {who}!");
        return;
    }

    let _id = random::<u8>();

    if _id == 0 {
        let _id = random::<u8>();
    }


    let mut sub  = sub_counter.lock().unwrap();

    if *sub == 0 {
        log::info!("init id {_id}");
        *sub = _id;
    }

    let sub = sub.clone();

    // we need something to defer what type the timer is, if it's an subathon timer we also have
    // to create a new thread that handles twitch and stuff
    let (mut tx, _rx) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    tokio::spawn(async move {
        loop {
            // In case of any websocket error, we exit.
            let mut text: String = String::from("");

            // check if we have the subathon timer and trait it different.

            if _type == "sub" /* && sub == _id */ { // maybe being renamed later in progress
                text = format!("{}", dec_time(6969, _id));
            }

            // send a tick to every timer every seconds, functionality is handled inside the timer itself
            if tx.send(Message::Text(text)).await.is_err() {
                log::info!("{who} broke connection");
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
            log::error!("Could not send Close due to {e}, probably it is ok? {_id}");
        }
    });
}

fn dec_time(id: i32, thread_id: u8) -> i32 {
    let sql = SQL.lock().expect("Can't lock");
    let sub = sub_counter.lock().unwrap().clone();
    let mut time = sql.get_time(id).unwrap().timer.to_seconds();

    time -= 1;

    if sub != thread_id {
        return time;
    }

    let timer = Timer {
        timer: Time::from(time),
        id,
        browser: false,
    };
    sql.create_timer(&timer);

    time

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
